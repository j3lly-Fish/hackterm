<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Theme } from '$lib/stores/theme';
  import type { AppSettings } from '$lib/stores/settings';

  export let theme: Theme;
  export let settings: AppSettings;
  export let host = '';
  export let port = 22;
  export let username = '';
  export let password = '';
  export let wsPort = 3010;
  export let onClose: () => void = () => {};

  let container: HTMLDivElement;
  let term: any = null;
  let sessionId = '';

  onMount(async () => {
    const { Terminal } = await import('@xterm/xterm');
    const { AttachAddon } = await import('@xterm/addon-attach');
    const { FitAddon } = await import('@xterm/addon-fit');

    term = new Terminal({
      fontSize: settings.termFontSize,
      fontFamily: theme.terminal.fontFamily,
      theme: {
        foreground: theme.terminal.foreground,
        background: theme.terminal.background,
      },
    });

    const fit = new FitAddon();
    term.loadAddon(fit);
    term.open(container);
    fit.fit();

    // Spawn SSH session on Rust side
    sessionId = await invoke<string>('spawn_ssh', {
      host,
      port,
      username,
      password,
      wsPort,
      cols: term.cols,
      rows: term.rows,
    });

    const ws = new WebSocket(`ws://127.0.0.1:${wsPort}`);
    ws.binaryType = 'arraybuffer';
    ws.onclose = () => onClose();
    const attach = new AttachAddon(ws);
    term.loadAddon(attach);
    term.focus();
  });

  onDestroy(async () => {
    if (sessionId) {
      await invoke('close_ssh', { sessionId }).catch(() => {});
    }
    term?.dispose();
  });
</script>

<div class="ssh_terminal" bind:this={container}></div>

<style>
  .ssh_terminal {
    width: 100%;
    height: 100%;
  }
</style>
