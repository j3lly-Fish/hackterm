<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;
  export let cwd: string = '';

  interface FileEntry {
    name: string;
    path: string;
    size: number;
    modified: number;
    kind: 'file' | 'dir' | 'symlink';
    is_symlink: boolean;
  }

  let entries: FileEntry[] = [];
  let currentPath = cwd || settings.cwd || '/';
  let unlisten: (() => void) | null = null;

  async function loadDir(path: string) {
    try {
      const raw = await invoke<FileEntry[]>('list_dir', { path });
      entries = raw
        .filter(e => !settings.hideDotfiles || !e.name.startsWith('.'))
        .sort((a, b) => {
          if (a.kind === 'dir' && b.kind !== 'dir') return -1;
          if (b.kind === 'dir' && a.kind !== 'dir') return 1;
          return a.name.localeCompare(b.name);
        });
      currentPath = path;

      // Watch directory for changes
      unlisten?.();
      await invoke('watch_dir', { path });
      unlisten = await listen<any>('fs-change', () => {
        loadDir(currentPath);
      });
    } catch (err) {
      console.error('list_dir failed:', err);
    }
  }

  function navigate(entry: FileEntry) {
    if (entry.kind === 'dir' || entry.kind === 'symlink') {
      loadDir(entry.path);
    } else {
      // TODO: open file in editor/viewer
    }
  }

  function goUp() {
    const parent = currentPath.split('/').slice(0, -1).join('/') || '/';
    loadDir(parent);
  }

  function fmtSize(bytes: number): string {
    if (bytes > 1e9) return (bytes / 1e9).toFixed(1) + 'G';
    if (bytes > 1e6) return (bytes / 1e6).toFixed(0) + 'M';
    if (bytes > 1e3) return (bytes / 1e3).toFixed(0) + 'K';
    return bytes + 'B';
  }

  onMount(() => {
    loadDir(currentPath);
    return () => unlisten?.();
  });

  // Follow terminal cwd
  $: if (cwd && cwd !== currentPath) loadDir(cwd);
</script>

<section
  id="filesystem"
  class:hideDotfiles={settings.hideDotfiles}
  class:list-view={settings.fsListView}
>
  <div class="fs_path">
    <button class="fs_up" on:click={goUp} title="Go up">↑</button>
    <span class="fs_cwd">{currentPath}</span>
  </div>
  <div class="fs_entries">
    {#each entries as e (e.path)}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        class="fs_entry fs_{e.kind}"
        class:symlink={e.is_symlink}
        on:click={() => navigate(e)}
        on:dblclick={() => navigate(e)}
      >
        <span class="fs_icon">{e.kind === 'dir' ? '▶' : '·'}</span>
        <span class="fs_name">{e.name}</span>
        {#if settings.fsListView}
          <span class="fs_size">{e.kind === 'file' ? fmtSize(e.size) : ''}</span>
        {/if}
      </div>
    {/each}
  </div>
</section>
