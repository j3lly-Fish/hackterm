<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { settings, shortcuts, saveSettings, saveShortcuts, configDir, type AppSettings, type Shortcut } from '$lib/stores/settings';
  import { get } from 'svelte/store';

  export let onClose: () => void;
  export let onThemeSwitch: (name: string) => void;
  export let onKeyboardSwitch: (layout: string) => void;

  let s: AppSettings = { ...get(settings)! };
  let scs: Shortcut[] = [...get(shortcuts)];
  let themes: string[] = [];
  let keyboards: string[] = [];
  let activeSection = 'appearance';

  onMount(async () => {
    const dir = get(configDir);
    try {
      const entries = await invoke<{ name: string; kind: string }[]>('list_dir', { path: dir + '/themes' });
      themes = entries.filter(e => e.kind === 'file' && e.name.endsWith('.json')).map(e => e.name.replace('.json', ''));
    } catch {}
    try {
      const entries = await invoke<{ name: string; kind: string }[]>('list_dir', { path: dir + '/kb_layouts' });
      keyboards = entries.filter(e => e.kind === 'file' && e.name.endsWith('.json')).map(e => e.name.replace('.json', ''));
    } catch {}
  });

  async function save() {
    await saveSettings(s);
    await saveShortcuts(scs);
    onClose();
  }

  const sections = ['appearance', 'terminal', 'system', 'misc'];
</script>

<div class="settings_overlay" on:click|self={onClose} role="dialog" aria-label="Settings">
  <div class="settings_panel" augmented-ui="tr-clip bl-clip exe">
    <header>
      <h2>SETTINGS</h2>
      <button class="close_btn" on:click={onClose}>×</button>
    </header>

    <nav>
      {#each sections as sec}
        <button class:active={activeSection === sec} on:click={() => activeSection = sec}>
          {sec.toUpperCase()}
        </button>
      {/each}
    </nav>

    <div class="settings_body">
      {#if activeSection === 'appearance'}
        <div class="field">
          <span class="field_label">THEME</span>
          <div class="theme_grid">
            {#each themes as t}
              <button
                class:selected={s.theme === t}
                on:click={() => { s.theme = t; onThemeSwitch(t); }}
              >{t}</button>
            {/each}
          </div>
        </div>
        <label class="field">
          <span class="field_label">KEYBOARD LAYOUT</span>
          <select bind:value={s.keyboard} on:change={() => onKeyboardSwitch(s.keyboard)}>
            {#each keyboards as kb}
              <option value={kb}>{kb}</option>
            {/each}
          </select>
        </label>

      {:else if activeSection === 'terminal'}
        <label class="field">
          <span class="field_label">SHELL</span>
          <input type="text" bind:value={s.shell} />
        </label>
        <label class="field">
          <span class="field_label">SHELL ARGS</span>
          <input type="text" bind:value={s.shellArgs} />
        </label>
        <label class="field">
          <span class="field_label">WORKING DIR</span>
          <input type="text" bind:value={s.cwd} />
        </label>
        <label class="field">
          <span class="field_label">FONT SIZE — {s.termFontSize}px</span>
          <input type="range" min="8" max="24" step="1" bind:value={s.termFontSize} />
        </label>
        <label class="field">
          <span class="field_label">WS PORT</span>
          <input type="number" min="1024" max="65535" bind:value={s.port} />
        </label>

      {:else if activeSection === 'system'}
        <label class="field row">
          <span class="field_label">AUDIO</span>
          <input type="checkbox" bind:checked={s.audio} />
        </label>
        <label class="field">
          <span class="field_label">VOLUME — {(s.audioVolume * 100).toFixed(0)}%</span>
          <input type="range" min="0" max="1" step="0.05" bind:value={s.audioVolume} />
        </label>
        <label class="field row">
          <span class="field_label">DISABLE FEEDBACK AUDIO</span>
          <input type="checkbox" bind:checked={s.disableFeedbackAudio} />
        </label>
        <label class="field">
          <span class="field_label">CLOCK FORMAT</span>
          <select bind:value={s.clockHours}>
            <option value={12}>12-hour</option>
            <option value={24}>24-hour</option>
          </select>
        </label>
        <label class="field">
          <span class="field_label">PING ADDRESS</span>
          <input type="text" bind:value={s.pingAddr} />
        </label>

      {:else if activeSection === 'misc'}
        <label class="field row"><span class="field_label">SKIP INTRO</span><input type="checkbox" bind:checked={s.nointro} /></label>
        <label class="field row"><span class="field_label">HIDE CURSOR</span><input type="checkbox" bind:checked={s.nocursor} /></label>
        <label class="field row"><span class="field_label">FORCE FULLSCREEN</span><input type="checkbox" bind:checked={s.forceFullscreen} /></label>
        <label class="field row"><span class="field_label">ALLOW WINDOWED</span><input type="checkbox" bind:checked={s.allowWindowed} /></label>
        <label class="field row"><span class="field_label">EXCLUDE THREADS FROM TOPLIST</span><input type="checkbox" bind:checked={s.excludeThreadsFromToplist} /></label>
        <label class="field row"><span class="field_label">HIDE DOTFILES</span><input type="checkbox" bind:checked={s.hideDotfiles} /></label>
        <label class="field row"><span class="field_label">FILESYSTEM LIST VIEW</span><input type="checkbox" bind:checked={s.fsListView} /></label>
        <label class="field row"><span class="field_label">SPLIT PANES</span><input type="checkbox" bind:checked={s.splitPanes} /></label>
      {/if}
    </div>

    <footer>
      <button class="btn_save" on:click={save}>SAVE</button>
      <button class="btn_cancel" on:click={onClose}>CANCEL</button>
    </footer>
  </div>
</div>

<style>
  .settings_overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 900;
  }
  .settings_panel {
    width: 580px;
    max-height: 82vh;
    background: rgba(0, 0, 0, 0.92);
    border: 1px solid rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    display: flex;
    flex-direction: column;
    color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    font-family: var(--font_main, monospace);
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    border-bottom: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.3);
  }
  header h2 { margin: 0; font-size: 0.9em; letter-spacing: 0.25em; }
  .close_btn {
    background: none;
    border: none;
    color: inherit;
    font-size: 1.4em;
    cursor: pointer;
    line-height: 1;
    padding: 0 4px;
  }
  nav {
    display: flex;
    gap: 2px;
    padding: 8px 16px;
    border-bottom: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.2);
  }
  nav button {
    background: none;
    border: 1px solid transparent;
    color: inherit;
    font-family: inherit;
    font-size: 0.7em;
    padding: 4px 12px;
    cursor: pointer;
    letter-spacing: 0.12em;
  }
  nav button.active {
    border-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.12);
  }
  .settings_body {
    padding: 14px 16px;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  .field.row {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
  .field_label {
    font-size: 0.68em;
    letter-spacing: 0.12em;
    opacity: 0.7;
  }
  input[type=text], input[type=number], select {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.08);
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.35);
    color: inherit;
    font-family: inherit;
    font-size: 0.85em;
    padding: 5px 8px;
    outline: none;
  }
  select option { background: #000; }
  input[type=range] {
    accent-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    width: 100%;
  }
  input[type=checkbox] {
    accent-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    width: 16px;
    height: 16px;
    cursor: pointer;
  }
  .theme_grid {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    margin-top: 2px;
  }
  .theme_grid button {
    background: none;
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.35);
    color: inherit;
    font-family: inherit;
    font-size: 0.7em;
    padding: 4px 12px;
    cursor: pointer;
    letter-spacing: 0.06em;
  }
  .theme_grid button.selected {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.18);
    border-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
  }
  footer {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    padding: 10px 16px;
    border-top: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.3);
  }
  footer button {
    background: none;
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.6);
    color: inherit;
    font-family: inherit;
    font-size: 0.72em;
    padding: 6px 22px;
    cursor: pointer;
    letter-spacing: 0.12em;
  }
  .btn_save {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.15) !important;
    border-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255)) !important;
  }
</style>
