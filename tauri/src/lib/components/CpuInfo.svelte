<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { cpuLoad, cpuStatic, cpuTemps } from '$lib/stores/sysinfo';

  // Smoothie chart
  let canvas: HTMLCanvasElement;
  let chart: any = null;
  let series: any = null;
  let unsub: (() => void) | null = null;

  onMount(async () => {
    const { SmoothieChart, TimeSeries } = await import('smoothie');

    // CSS variables don't resolve inside canvas context — read computed values instead
    const cs = getComputedStyle(document.documentElement);
    const r = cs.getPropertyValue('--color_r').trim() || '0';
    const g = cs.getPropertyValue('--color_g').trim() || '200';
    const b = cs.getPropertyValue('--color_b').trim() || '255';
    const lineColor = `rgb(${r},${g},${b})`;
    const fillColor = `rgba(${r},${g},${b},0.25)`;
    const gridColor = `rgba(${r},${g},${b},0.12)`;

    series = new TimeSeries();
    chart = new SmoothieChart({
      millisPerPixel: 50,
      grid: { fillStyle: 'rgba(0,0,0,0.4)', strokeStyle: gridColor, verticalSections: 4 },
      labels: { disabled: true },
      maxValue: 100,
      minValue: 0,
    });
    chart.addTimeSeries(series, { strokeStyle: lineColor, fillStyle: fillColor, lineWidth: 2 });
    chart.streamTo(canvas, 1000);

    unsub = cpuLoad.subscribe(v => {
      series?.append(Date.now(), v.global);
    });
  });

  onDestroy(() => { chart?.stop(); unsub?.(); });
</script>

<div class="mod_cpuinfo">
  <h4 class="mod_title">CPU</h4>
  <canvas bind:this={canvas} width="200" height="60"></canvas>
  <p class="cpu_brand">{$cpuStatic?.brand ?? '...'}</p>
  <p class="cpu_usage">{$cpuLoad.global.toFixed(1)}%</p>
  <div class="cpu_cores">
    {#each $cpuLoad.per_core as v, i}
      <div class="core_bar" title="Core {i}: {v.toFixed(0)}%">
        <div class="core_fill" style="height: {v}%"></div>
      </div>
    {/each}
  </div>
  {#if $cpuTemps.length > 0}
    <p class="cpu_temp">{$cpuTemps[0].temperature.toFixed(0)}°C</p>
  {/if}
</div>
