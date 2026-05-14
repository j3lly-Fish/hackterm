<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { exit } from '@tauri-apps/plugin-process';
  import { register, unregisterAll } from '@tauri-apps/plugin-global-shortcut';

  // Stores
  import { settings, shortcuts, loadSettings, saveSettings, type AppSettings, type Shortcut } from '$lib/stores/settings';
  import { loadTheme, applyTheme, currentTheme, type Theme } from '$lib/stores/theme';
  import { startPolling, stopPolling } from '$lib/stores/sysinfo';
  import { tabs, activeTab, spawnMainTerminal, spawnExtraTerminal, closeTab } from '$lib/stores/terminals';

  // Components
  import BootScreen from '$lib/components/BootScreen.svelte';
  import Terminal from '$lib/components/Terminal.svelte';
  import Clock from '$lib/components/Clock.svelte';
  import CpuInfo from '$lib/components/CpuInfo.svelte';
  import RamWatcher from '$lib/components/RamWatcher.svelte';
  import TopList from '$lib/components/TopList.svelte';
  import HardwareInspector from '$lib/components/HardwareInspector.svelte';
  import NetStat from '$lib/components/NetStat.svelte';
  import ConnInfo from '$lib/components/ConnInfo.svelte';
  import LocationGlobe from '$lib/components/LocationGlobe.svelte';
  import Filesystem from '$lib/components/Filesystem.svelte';
  import Keyboard from '$lib/components/Keyboard.svelte';
  import AudioFx from '$lib/components/AudioFx.svelte';
  import AlertManager from '$lib/components/AlertManager.svelte';
  import Modal from '$lib/components/Modal.svelte';
  import UpdateChecker from '$lib/components/UpdateChecker.svelte';
  import SplitPane from '$lib/components/SplitPane.svelte';
  import FuzzyFinder from '$lib/components/FuzzyFinder.svelte';
  import SettingsPanel from '$lib/components/SettingsPanel.svelte';
  import SshDialog from '$lib/components/SshDialog.svelte';
  import SshTerminal from '$lib/components/SshTerminal.svelte';

  // ──────────────────────── State ────────────────────────

  let booting = true;
  let uiReady = false;
  let s: AppSettings | null = null;
  let theme: Theme | null = null;
  let termRefs: Record<number, Terminal> = {};
  let audioFx: AudioFx;

  // Overlays
  let modalVisible = false;
  let modalType: 'info' | 'error' | 'warning' | 'confirm' = 'error';
  let modalTitle = '';
  let modalMsg = '';
  let fuzzyVisible = false;
  let settingsOpen = false;
  let sshDialogVisible = false;
  let sshActive = false;
  let sshParams: { host: string; port: number; username: string; password: string; wsPort: number } | null = null;

  // Cleanup handles
  let unlistenClose: (() => void) | null = null;
  const unsubSettings = settings.subscribe(v => { s = v; });
  const unsubTheme = currentTheme.subscribe(v => { theme = v; });

  // ──────────────────────── Boot ────────────────────────

  onMount(async () => {
    window.onerror = (msg, _url, line, col, err) => {
      if (isAudioError(String(msg))) return;
      showError(String(err ?? 'Error'), `${msg}\n\nat line ${line}:${col}`);
    };
    window.onunhandledrejection = (e) => {
      if (isAudioError(String(e.reason))) return;
      showError('Unhandled async error', String(e.reason));
    };

    await loadSettings();
    await invoke('mirror_assets');

    const curSettings = get(settings)!;

    // Restore window state
    const win = getCurrentWindow();
    const winState = await invoke<{ fullscreen: boolean }>('load_window_state');
    if (curSettings.forceFullscreen || winState.fullscreen) {
      await win.setFullscreen(true);
    }

    // Save window state when OS close button is pressed
    unlistenClose = await win.onCloseRequested(async (event) => {
      event.preventDefault();
      await saveWindowStateAndExit();
    });

    const dir = await invoke<string>('get_config_dir');
    const curTheme = await loadTheme(curSettings.theme, dir);
    if (curTheme) applyTheme(curTheme, curSettings.nocursor);

    if (curSettings.nointro) {
      finishBoot();
    }
    // Otherwise BootScreen calls finishBoot via on:complete
  });

  async function saveWindowStateAndExit() {
    const win = getCurrentWindow();
    const isFs = await win.isFullscreen().catch(() => true);
    await invoke('save_window_state', { state: { fullscreen: isFs } });
    exit(0);
  }

  async function finishBoot() {
    booting = false;
    await new Promise(r => requestAnimationFrame(r));

    const curSettings = get(settings)!;
    startPolling();

    try {
      await spawnMainTerminal(
        curSettings.shell,
        curSettings.cwd,
        curSettings.port,
        80,
        24,
      );
    } catch (err) {
      // Show error but don't block the UI — terminal can be retried
      showError('Terminal spawn failed', String(err));
    }

    try {
      await registerShortcuts();
    } catch {}

    uiReady = true;
  }

  // ──────────────────────── Keyboard shortcuts ────────────────────────

  async function registerShortcuts() {
    const scs = get(shortcuts);
    await unregisterAll();
    for (const sc of scs) {
      const modifiers: string[] = [];
      if (sc.ctrlKey) modifiers.push('Control');
      if (sc.shiftKey) modifiers.push('Shift');
      if (sc.altKey) modifiers.push('Alt');
      const combo = [...modifiers, sc.key].join('+');
      try {
        await register(combo, () => useShortcut(sc.name));
      } catch {}
    }
  }

  async function useShortcut(name: string) {
    switch (name) {
      case 'COPY':
        document.execCommand('copy');
        break;
      case 'PASTE': {
        const tab = get(tabs)[get(activeTab)];
        if (tab) {
          const text = await navigator.clipboard.readText().catch(() => '');
          if (text) {
            const ws = new WebSocket(`ws://127.0.0.1:${tab.port}`);
            ws.onopen = () => { ws.send(text); ws.close(); };
          }
        }
        break;
      }
      case 'NEXT_TAB':
        focusNextTab(1);
        break;
      case 'PREVIOUS_TAB':
        focusNextTab(-1);
        break;
      case 'KILL_TAB': {
        const idx = get(activeTab);
        if (idx > 0) closeTab(idx);
        break;
      }
      case 'SETTINGS':
        settingsOpen = !settingsOpen;
        break;
      case 'SSH_CONNECT':
        sshDialogVisible = true;
        break;
      case 'CLOSE':
        await saveWindowStateAndExit();
        break;
    }
  }

  function focusNextTab(dir: number) {
    const list = get(tabs);
    const cur = get(activeTab);
    for (let i = 1; i <= list.length; i++) {
      const next = (cur + dir * i + list.length) % list.length;
      if (list[next]) {
        activeTab.set(next);
        termRefs[next]?.focus();
        break;
      }
    }
  }

  // ──────────────────────── Tabs ────────────────────────

  async function activateTab(idx: number) {
    const list = get(tabs);
    if (idx === get(activeTab)) return;
    if (list[idx]) {
      activeTab.set(idx);
      await new Promise(r => requestAnimationFrame(r));
      termRefs[idx]?.fit();
      termRefs[idx]?.focus();
    } else if (idx > 0) {
      const curSettings = get(settings)!;
      await spawnExtraTerminal(idx, curSettings.shell, curSettings.cwd, curSettings.port);
      activeTab.set(idx);
      await new Promise(r => requestAnimationFrame(r));
      termRefs[idx]?.fit();
      termRefs[idx]?.focus();
    }
  }

  // ──────────────────────── Theme / keyboard switch ────────────────────────

  async function switchTheme(themeName: string) {
    const dir = await invoke<string>('get_config_dir');
    const newTheme = await loadTheme(themeName, dir);
    if (newTheme) {
      applyTheme(newTheme, get(settings)?.nocursor ?? false);
      const cur = get(settings)!;
      await saveSettings({ ...cur, theme: themeName });
    }
  }

  async function switchKeyboard(layout: string) {
    const cur = get(settings)!;
    await saveSettings({ ...cur, keyboard: layout });
    settings.update(v => v ? { ...v, keyboard: layout } : v);
  }

  // ──────────────────────── SSH ────────────────────────

  function handleSshConnect(host: string, port: number, username: string, password: string, wsPort: number) {
    sshDialogVisible = false;
    sshParams = { host, port, username, password, wsPort };
    sshActive = true;
  }

  function handleSshClose() {
    sshActive = false;
    sshParams = null;
  }

  // ──────────────────────── Error / modal ────────────────────────

  function isAudioError(msg: string): boolean {
    const m = msg.toLowerCase();
    return m.includes('audio') || m.includes('audiocontext') ||
           m.includes('invalidstateerror') || m.includes('howler') ||
           m.includes('webaudio') || m.includes('sound');
  }

  function showError(title: string, message: string) {
    modalType = 'error';
    modalTitle = title;
    modalMsg = message;
    modalVisible = true;
  }

  // ──────────────────────── Cleanup ────────────────────────

  onDestroy(() => {
    unsubSettings();
    unsubTheme();
    stopPolling();
    unregisterAll();
    unlistenClose?.();
  });
</script>

<!-- CSS imports -->
<svelte:head>
  <link rel="stylesheet" href="/assets/css/augmented-ui.css" />
  <link rel="stylesheet" href="/assets/css/main.css" />
  <link rel="stylesheet" href="/assets/css/modal.css" />
  <link rel="stylesheet" href="/assets/css/boot_screen.css" />
  <link rel="stylesheet" href="/assets/css/media_player.css" />
  <link rel="stylesheet" href="/assets/css/main_shell.css" />
  <link rel="stylesheet" href="/assets/css/filesystem.css" />
  <link rel="stylesheet" href="/assets/css/keyboard.css" />
  <link rel="stylesheet" href="/assets/css/mod_column.css" />
  <link rel="stylesheet" href="/assets/css/mod_clock.css" />
  <link rel="stylesheet" href="/assets/css/mod_sysinfo.css" />
  <link rel="stylesheet" href="/assets/css/mod_hardwareInspector.css" />
  <link rel="stylesheet" href="/assets/css/mod_cpuinfo.css" />
  <link rel="stylesheet" href="/assets/css/mod_netstat.css" />
  <link rel="stylesheet" href="/assets/css/mod_conninfo.css" />
  <link rel="stylesheet" href="/assets/css/mod_globe.css" />
  <link rel="stylesheet" href="/assets/css/mod_ramwatcher.css" />
  <link rel="stylesheet" href="/assets/css/mod_toplist.css" />
  <link rel="stylesheet" href="/assets/css/mod_fuzzyFinder.css" />
  <link rel="stylesheet" href="/assets/css/extra_ratios.css" />
</svelte:head>

<!-- Boot screen -->
{#if booting && theme}
  <BootScreen
    {theme}
    nointro={s?.nointro ?? false}
    on:complete={finishBoot}
  />
{/if}

<!-- Audio + alerts (always mounted once settings ready) -->
{#if s}
  <AudioFx bind:this={audioFx} settings={s} />
  <AlertManager settings={s} playAlert={() => audioFx?.playAlarm()} />
{/if}

<!-- Main UI -->
{#if uiReady && s && theme}
  <div id="ui_root">
    <!-- Left column -->
    <section class="mod_column activated" id="mod_column_left">
      <h3 class="title"><p>PANEL</p><p>SYSTEM</p></h3>
      <Clock settings={s} />
      <HardwareInspector />
      <CpuInfo />
      <RamWatcher />
      <TopList settings={s} />
    </section>

    <!-- Terminal -->
    <section id="main_shell" augmented-ui="bl-clip tr-clip exe">
      <h3 class="title"><p>TERMINAL</p><p>MAIN SHELL</p></h3>

      <ul id="main_shell_tabs">
        {#each $tabs as tab, i}
          <li
            id="shell_tab{i}"
            class:active={$activeTab === i}
            on:click={() => activateTab(i)}
          >
            <p>{tab?.label ?? 'EMPTY'}</p>
          </li>
        {/each}
      </ul>

      <div id="main_shell_innercontainer">
        <SplitPane enabled={s.splitPanes} ratio={s.splitRatio}>
          <svelte:fragment slot="left">
            {#each $tabs as tab, i}
              {#if tab}
                <Terminal
                  bind:this={termRefs[i]}
                  port={tab.port}
                  pid={tab.pid}
                  index={i}
                  active={$activeTab === i}
                  settings={s}
                  {theme}
                />
              {/if}
            {/each}
          </svelte:fragment>
        </SplitPane>
      </div>
    </section>

    <!-- Filesystem -->
    <Filesystem
      settings={s}
      cwd={$tabs[$activeTab]?.cwd ?? s.cwd}
    />

    <!-- On-screen keyboard -->
    <Keyboard
      settings={s}
      onKeyPress={(cmd) => {
        const tab = $tabs[$activeTab];
        if (tab) {
          const ws = new WebSocket(`ws://127.0.0.1:${tab.port}`);
          ws.onopen = () => { ws.send(cmd); ws.close(); };
        }
      }}
    />

    <!-- Right column -->
    <section class="mod_column activated" id="mod_column_right">
      <h3 class="title"><p>PANEL</p><p>NETWORK</p></h3>
      <NetStat pingAddr={s.pingAddr} />
      <LocationGlobe {theme} />
      <ConnInfo />
    </section>

    <!-- Overlays -->
    <FuzzyFinder
      bind:visible={fuzzyVisible}
      on:select={(e) => {
        fuzzyVisible = false;
      }}
    />

    {#if settingsOpen}
      <SettingsPanel
        onClose={() => { settingsOpen = false; }}
        onThemeSwitch={switchTheme}
        onKeyboardSwitch={switchKeyboard}
      />
    {/if}

    {#if sshDialogVisible}
      <SshDialog
        profiles={s.sshProfiles ?? []}
        onConnect={handleSshConnect}
        onClose={() => { sshDialogVisible = false; }}
      />
    {/if}

    {#if sshActive && sshParams && theme}
      <div class="ssh_overlay_container">
        <SshTerminal
          {theme}
          settings={s}
          host={sshParams.host}
          port={sshParams.port}
          username={sshParams.username}
          password={sshParams.password}
          wsPort={sshParams.wsPort}
          onClose={handleSshClose}
        />
        <button class="ssh_close_btn" on:click={handleSshClose}>CLOSE SSH</button>
      </div>
    {/if}

    <UpdateChecker />
    <Modal
      bind:visible={modalVisible}
      type={modalType}
      title={modalTitle}
      message={modalMsg}
    />
  </div>
{/if}

<style>
  /* Full-viewport layout root — mirrors body{} from main.css but with
     position:fixed so that:
       - absolutely-positioned columns (.mod_column) resolve left/right correctly
       - percentage heights (main_shell 60.3%, etc.) have a definite ancestor */
  :global(#ui_root) {
    position: fixed;
    inset: 0;
    overflow: hidden;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    padding-top: 1.85vh;
    user-select: none;
    color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
  }
  :global(#ui_root.solidBackground) {
    background: var(--color_light_black);
  }

  .ssh_overlay_container {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.92);
    z-index: 800;
    display: flex;
    flex-direction: column;
  }
  .ssh_close_btn {
    position: absolute;
    top: 12px;
    right: 16px;
    background: none;
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.6);
    color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    font-family: var(--font_main, monospace);
    font-size: 0.72em;
    padding: 5px 14px;
    cursor: pointer;
    letter-spacing: 0.1em;
    z-index: 801;
  }
</style>
