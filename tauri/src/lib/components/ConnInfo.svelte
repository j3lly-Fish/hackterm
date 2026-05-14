<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { networkInterfaces } from '$lib/stores/sysinfo';
  import { invoke } from '@tauri-apps/api/core';

  let canvas: HTMLCanvasElement;
  let chart: any = null;
  let rxSeries: any = null;
  let txSeries: any = null;
  let unsub: (() => void) | null = null;

  export let ifaceName = '';

  $: activeIface = $networkInterfaces.find(i => i.name === ifaceName)
    ?? $networkInterfaces.find(i => i.ip4.length > 0 && !i.name.startsWith('lo'))
    ?? null;

  function fmtSpeed(bps: number): string {
    if (bps > 1e6) return (bps / 1e6).toFixed(1) + ' MB/s';
    if (bps > 1e3) return (bps / 1e3).toFixed(0) + ' KB/s';
    return bps + ' B/s';
  }

  onMount(async () => {
    const { SmoothieChart, TimeSeries } = await import('smoothie');
    rxSeries = new TimeSeries();
    txSeries = new TimeSeries();
    chart = new SmoothieChart({
      millisPerPixel: 50,
      grid: { fillStyle: 'transparent', strokeStyle: 'rgba(255,255,255,0.05)', verticalSections: 3 },
      labels: { disabled: true },
    });
    chart.addTimeSeries(rxSeries, { strokeStyle: 'rgba(0,200,255,0.9)', lineWidth: 2 });
    chart.addTimeSeries(txSeries, { strokeStyle: 'rgba(255,100,0,0.8)', lineWidth: 1 });
    chart.streamTo(canvas, 2000);

    unsub = networkInterfaces.subscribe(ifaces => {
      const iface = ifaces.find(i => i.name === ifaceName)
        ?? ifaces.find(i => i.ip4.length > 0 && !i.name.startsWith('lo'));
      if (iface) {
        rxSeries?.append(Date.now(), iface.rx_speed);
        txSeries?.append(Date.now(), iface.tx_speed);
      }
    });
  });

  onDestroy(() => { chart?.stop(); unsub?.(); });
</script>

<div class="mod_conninfo">
  <h4 class="mod_title">TRAFFIC</h4>
  <canvas bind:this={canvas} width="200" height="50"></canvas>
  {#if activeIface}
    <p>↓ {fmtSpeed(activeIface.rx_speed)} | ↑ {fmtSpeed(activeIface.tx_speed)}</p>
  {:else}
    <p>No active interface</p>
  {/if}
</div>
