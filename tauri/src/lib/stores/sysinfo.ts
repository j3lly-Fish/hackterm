import { writable, readable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// ──────────────────────── Types ────────────────────────

export interface CpuLoad { global: number; per_core: number[] }
export interface CpuStatic { brand: string; cores: number; logical: number; frequency: number }
export interface CpuTemp { label: string; temperature: number }
export interface Memory { total: number; used: number; free: number; swap_total: number; swap_used: number }
export interface ProcessInfo { pid: number; name: string; cpu: number; memory: number; status: string }
export interface NetworkInterface { name: string; ip4: string[]; ip6: string[]; mac: string; rx_bytes: number; tx_bytes: number; rx_speed: number; tx_speed: number }
export interface NetworkStats { name: string; rx: number; tx: number; rx_total: number; tx_total: number }
export interface BatteryInfo { charge: number; state: string; time_to_full?: number; time_to_empty?: number }
export interface SystemInfo { hostname: string; os_name: string; os_version: string; kernel_version: string; uptime: number; boot_time: number }
export interface HardwareInfo { cpu_brand: string; cpu_cores: number; total_memory: number; disks: { name: string; mount: string; total: number; available: number }[] }
export interface NetworkConnection { protocol: string; local_addr: string; local_port: number; remote_addr: string; remote_port: number; state: string; pid?: number }

// ──────────────────────── Writable stores ────────────────────────

export const cpuLoad = writable<CpuLoad>({ global: 0, per_core: [] });
export const cpuStatic = writable<CpuStatic | null>(null);
export const cpuTemps = writable<CpuTemp[]>([]);
export const memory = writable<Memory | null>(null);
export const processes = writable<ProcessInfo[]>([]);
export const networkInterfaces = writable<NetworkInterface[]>([]);
export const battery = writable<BatteryInfo | null>(null);
export const systemInfo = writable<SystemInfo | null>(null);
export const hardwareInfo = writable<HardwareInfo | null>(null);
export const networkConnections = writable<NetworkConnection[]>([]);

// ──────────────────────── Polling helpers ────────────────────────

let cpuPollId: ReturnType<typeof setInterval> | null = null;
let memPollId: ReturnType<typeof setInterval> | null = null;
let procPollId: ReturnType<typeof setInterval> | null = null;
let netPollId: ReturnType<typeof setInterval> | null = null;

export function startPolling() {
  stopPolling();

  // CPU every 1 s
  cpuPollId = setInterval(async () => {
    const v = await invoke<CpuLoad>('get_cpu_load').catch(() => null);
    if (v) cpuLoad.set(v);
    const t = await invoke<CpuTemp[]>('get_cpu_temp').catch(() => []);
    cpuTemps.set(t);
  }, 1000);

  // Memory every 1.5 s
  memPollId = setInterval(async () => {
    const v = await invoke<Memory>('get_memory').catch(() => null);
    if (v) memory.set(v);
    const b = await invoke<BatteryInfo | null>('get_battery').catch(() => null);
    battery.set(b);
  }, 1500);

  // Processes every 3 s
  procPollId = setInterval(async () => {
    const v = await invoke<ProcessInfo[]>('get_processes').catch(() => []);
    processes.set(v);
  }, 3000);

  // Network interfaces every 2 s
  netPollId = setInterval(async () => {
    const v = await invoke<NetworkInterface[]>('get_network_interfaces').catch(() => []);
    networkInterfaces.set(v);
  }, 2000);

  // Load static info once
  invoke<CpuStatic>('get_cpu_static').then(cpuStatic.set).catch(() => {});
  invoke<SystemInfo>('get_system_info').then(systemInfo.set).catch(() => {});
  invoke<HardwareInfo>('get_hardware_info').then(hardwareInfo.set).catch(() => {});
}

export function stopPolling() {
  if (cpuPollId !== null) { clearInterval(cpuPollId); cpuPollId = null; }
  if (memPollId !== null) { clearInterval(memPollId); memPollId = null; }
  if (procPollId !== null) { clearInterval(procPollId); procPollId = null; }
  if (netPollId !== null) { clearInterval(netPollId); netPollId = null; }
}
