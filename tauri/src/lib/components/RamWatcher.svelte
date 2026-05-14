<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { memory } from '$lib/stores/sysinfo';

  let canvas: HTMLCanvasElement;
  let chart: any = null;
  let ramSeries: any = null;
  let swapSeries: any = null;
  let unsub: (() => void) | null = null;

  function fmtBytes(b: number): string {
    if (b > 1e9) return (b / 1e9).toFixed(1) + ' GB';
    return (b / 1e6).toFixed(0) + ' MB';
  }

  onMount(async () => {
    const { SmoothieChart, TimeSeries } = await import('smoothie');

    const cs = getComputedStyle(document.documentElement);
    const r = cs.getPropertyValue('--color_r').trim() || '0';
    const g = cs.getPropertyValue('--color_g').trim() || '200';
    const b = cs.getPropertyValue('--color_b').trim() || '255';
    const lineColor = `rgb(${r},${g},${b})`;
    const fillColor = `rgba(${r},${g},${b},0.25)`;
    const gridColor = `rgba(${r},${g},${b},0.12)`;

    ramSeries = new TimeSeries();
    swapSeries = new TimeSeries();
    chart = new SmoothieChart({
      millisPerPixel: 50,
      grid: { fillStyle: 'rgba(0,0,0,0.4)', strokeStyle: gridColor, verticalSections: 3 },
      labels: { disabled: true },
      maxValue: 100,
      minValue: 0,
    });
    chart.addTimeSeries(ramSeries, { strokeStyle: lineColor, fillStyle: fillColor, lineWidth: 2 });
    chart.addTimeSeries(swapSeries, { strokeStyle: 'rgba(255,140,0,0.85)', fillStyle: 'rgba(255,140,0,0.15)', lineWidth: 1.5 });
    chart.streamTo(canvas, 1500);

    unsub = memory.subscribe(v => {
      if (!v) return;
      ramSeries?.append(Date.now(), (v.used / v.total) * 100);
      if (v.swap_total > 0)
        swapSeries?.append(Date.now(), (v.swap_used / v.swap_total) * 100);
    });
  });

  onDestroy(() => { chart?.stop(); unsub?.(); });
</script>

<div class="mod_ramwatcher">
  <h4 class="mod_title">MEMORY</h4>
  <canvas bind:this={canvas} width="200" height="50"></canvas>
  {#if $memory}
    <p>RAM: {fmtBytes($memory.used)} / {fmtBytes($memory.total)}</p>
    {#if $memory.swap_total > 0}
      <p>SWP: {fmtBytes($memory.swap_used)} / {fmtBytes($memory.swap_total)}</p>
    {/if}
  {:else}
    <p>...</p>
  {/if}
</div>
