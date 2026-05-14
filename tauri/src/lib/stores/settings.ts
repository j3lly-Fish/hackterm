import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface AlertsConfig {
  enabled: boolean;
  cpuThreshold: number;
  ramThreshold: number;
  tempThreshold: number;
  cooldownSeconds: number;
}

export interface AppSettings {
  shell: string;
  shellArgs: string;
  cwd: string;
  keyboard: string;
  theme: string;
  termFontSize: number;
  audio: boolean;
  audioVolume: number;
  disableFeedbackAudio: boolean;
  clockHours: number;
  pingAddr: string;
  port: number;
  nointro: boolean;
  nocursor: boolean;
  forceFullscreen: boolean;
  allowWindowed: boolean;
  excludeThreadsFromToplist: boolean;
  hideDotfiles: boolean;
  fsListView: boolean;
  experimentalGlobeFeatures: boolean;
  experimentalFeatures: boolean;
  splitPanes: boolean;
  splitRatio: number;
  sshProfiles: SshProfile[];
  monitor?: number;
  alerts: AlertsConfig;
  username?: string;
}

export interface SshProfile {
  name: string;
  host: string;
  port: number;
  username: string;
  authType: 'password' | 'key';
  keyPath?: string;
}

export interface Shortcut {
  name: string;
  key: string;
  ctrlKey: boolean;
  shiftKey: boolean;
  altKey?: boolean;
}

export const settings = writable<AppSettings | null>(null);
export const shortcuts = writable<Shortcut[]>([]);
export const configDir = writable<string>('');
export const settingsLoaded = writable(false);

export async function loadSettings(): Promise<void> {
  try {
    const [s, sc, dir] = await Promise.all([
      invoke<AppSettings>('load_settings'),
      invoke<Shortcut[]>('load_shortcuts'),
      invoke<string>('get_config_dir'),
    ]);
    settings.set(s);
    shortcuts.set(sc);
    configDir.set(dir);
    settingsLoaded.set(true);
  } catch (err) {
    console.error('Failed to load settings:', err);
  }
}

export async function saveSettings(s: AppSettings): Promise<void> {
  await invoke('save_settings', { settings: s });
  settings.set(s);
}

export async function saveShortcuts(sc: Shortcut[]): Promise<void> {
  await invoke('save_shortcuts', { shortcuts: sc });
  shortcuts.set(sc);
}
