<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { networkInterfaces, battery } from '$lib/stores/sysinfo';
  import { invoke } from '@tauri-apps/api/core';

  let externalIp = '';
  let pingMs: number | null = null;

  export let pingAddr: string;

  async function refreshIp() {
    externalIp = await invoke<string>('get_external_ip').catch(() => '—');
  }

  async function refreshPing() {
    const r = await invoke<{ success: boolean; latency_ms?: number }>('ping', { addr: pingAddr }).catch(() => null);
    pingMs = r?.latency_ms ?? null;
  }

  onMount(() => {
    refreshIp();
    refreshPing();
    const id = setInterval(refreshPing, 5000);
    return () => clearInterval(id);
  });

  $: primaryIface = $networkInterfaces.find(i => i.ip4.length > 0 && !i.name.startsWith('lo'));
</script>

<div class="mod_netstat">
  <h4 class="mod_title">NETWORK</h4>
  {#if primaryIface}
    <p><span class="label">IFACE</span>{primaryIface.name}</p>
    <p><span class="label">IP4</span>{primaryIface.ip4[0] ?? '—'}</p>
    <p><span class="label">MAC</span>{primaryIface.mac}</p>
  {/if}
  <p><span class="label">EXT IP</span>{externalIp || '...'}</p>
  <p><span class="label">PING</span>{pingMs !== null ? pingMs + 'ms' : '...'}</p>
  {#if $battery}
    <p><span class="label">BAT</span>{$battery.charge.toFixed(0)}% ({$battery.state})</p>
  {/if}
</div>
