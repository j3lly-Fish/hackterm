<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { AppSettings } from '$lib/stores/settings';
  import type { Theme } from '$lib/stores/theme';
  import { setTabLabel, setTabCwd } from '$lib/stores/terminals';

  export let port: number;
  export let index: number;
  export let pid: number = 0;
  export let active: boolean = false;
  export let settings: AppSettings;
  export let theme: Theme;

  let container: HTMLElement;
  let term: import('@xterm/xterm').Terminal | null = null;
  let fitAddon: import('@xterm/addon-fit').FitAddon | null = null;
  let ws: WebSocket | null = null;
  let cwdPollId: ReturnType<typeof setInterval> | null = null;

  export function fit() {
    fitAddon?.fit();
    if (term) {
      invoke('resize_terminal', { port, cols: term.cols, rows: term.rows }).catch(() => {});
    }
  }

  export function focus() {
    term?.focus();
  }

  onMount(async () => {
    const { Terminal } = await import('@xterm/xterm');
    const { AttachAddon } = await import('@xterm/addon-attach');
    const { FitAddon } = await import('@xterm/addon-fit');
    const { WebglAddon } = await import('@xterm/addon-webgl');

    const themeColor = `rgb(${theme.colors.r}, ${theme.colors.g}, ${theme.colors.b})`;

    term = new Terminal({
      fontSize: settings.termFontSize,
      fontFamily: theme.terminal.fontFamily,
      cursorStyle: theme.terminal.cursorStyle as any,
      theme: {
        foreground: theme.terminal.foreground,
        background: theme.terminal.background,
        cursor: theme.terminal.cursor,
        cursorAccent: theme.terminal.cursorAccent,
        selectionBackground: theme.terminal.selection,
      },
      allowTransparency: true,
      scrollback: 5000,
    });

    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);

    term.open(container);
    fitAddon.fit();

    // WebGL renderer
    try {
      const webgl = new WebglAddon();
      term.loadAddon(webgl);
    } catch {}

    // Connect to PTY via WebSocket
    ws = new WebSocket(`ws://127.0.0.1:${port}`);
    ws.binaryType = 'arraybuffer';

    const attach = new AttachAddon(ws);
    term.loadAddon(attach);

    ws.onopen = () => {
      if (active) term?.focus();
    };

    ws.onclose = () => {
      // Terminal closed — notify parent
    };

    // Resize observer
    const ro = new ResizeObserver(() => fit());
    ro.observe(container);

    // CWD polling (check every 2 s)
    cwdPollId = setInterval(async () => {
      if (pid > 0) {
        const cwd = await invoke<string>('get_terminal_cwd', { pid }).catch(() => '');
        if (cwd) setTabCwd(index, cwd);
      }
    }, 2000);

    term.writeln(`\x1b[1mWelcome to eDEX-UI v2.3.0\x1b[0m`);
  });

  onDestroy(() => {
    if (cwdPollId !== null) clearInterval(cwdPollId);
    ws?.close();
    term?.dispose();
  });

  $: if (active && term) {
    tick().then(() => { fit(); focus(); });
  }
</script>

<pre
  class="terminal-container"
  class:active
  bind:this={container}
  id="terminal{index}"
></pre>

<style>
  .terminal-container {
    display: none;
    width: 100%;
    height: 100%;
    overflow: hidden;
    margin: 0;
    padding: 0;
  }
  .terminal-container.active {
    display: block;
  }
  :global(.xterm) {
    height: 100%;
  }
</style>
