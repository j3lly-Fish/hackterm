import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface TerminalTab {
  index: number;
  port: number;
  pid: number;
  label: string;
  active: boolean;
  cwd: string;
}

export const tabs = writable<(TerminalTab | null)[]>([null, null, null, null, null]);
export const activeTab = writable<number>(0);

/** Spawn the main terminal (tab 0) */
export async function spawnMainTerminal(
  shell: string,
  cwd: string,
  port: number,
  cols = 80,
  rows = 24,
): Promise<void> {
  const result = await invoke<{ pid: number; port: number }>('spawn_terminal', {
    shell,
    args: [],
    cwd,
    port,
    cols,
    rows,
  });

  const tab: TerminalTab = {
    index: 0,
    port: result.port,
    pid: result.pid,
    label: 'MAIN SHELL',
    active: true,
    cwd,
  };

  tabs.update(t => {
    const next = [...t];
    next[0] = tab;
    return next;
  });
  activeTab.set(0);
}

/** Spawn an extra terminal tab (tabs 1-4) */
export async function spawnExtraTerminal(
  index: number,
  shell: string,
  cwd: string,
  basePort: number,
  cols = 80,
  rows = 24,
): Promise<number> {
  const result = await invoke<{ pid: number; port: number }>('spawn_terminal', {
    shell,
    args: [],
    cwd,
    port: basePort + index * 2,
    cols,
    rows,
  });

  const tab: TerminalTab = {
    index,
    port: result.port,
    pid: result.pid,
    label: `#${index + 1}`,
    active: false,
    cwd,
  };

  tabs.update(t => {
    const next = [...t];
    next[index] = tab;
    return next;
  });
  return result.port;
}

export async function closeTab(index: number): Promise<void> {
  const t = get(tabs)[index];
  if (t) {
    await invoke('close_terminal', { port: t.port }).catch(() => {});
    tabs.update(list => {
      const next = [...list];
      next[index] = null;
      return next;
    });
  }
  // Focus previous active tab
  if (get(activeTab) === index) {
    const list = get(tabs);
    for (let i = index - 1; i >= 0; i--) {
      if (list[i]) { activeTab.set(i); break; }
    }
  }
}

export function setTabLabel(index: number, label: string) {
  tabs.update(list => {
    const next = [...list];
    if (next[index]) {
      next[index] = { ...next[index]!, label };
    }
    return next;
  });
}

export function setTabCwd(index: number, cwd: string) {
  tabs.update(list => {
    const next = [...list];
    if (next[index]) {
      next[index] = { ...next[index]!, cwd };
    }
    return next;
  });
}
