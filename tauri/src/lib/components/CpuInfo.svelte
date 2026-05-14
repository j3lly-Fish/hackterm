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
    series = new TimeSeries();
    chart = new SmoothieChart({
      millisPerPixel: 50,
      grid: { fillStyle: 'transparent', strokeStyle: 'rgba(255,255,255,0.05)', verticalSections: 4 },
      labels: { disabled: true },
      maxValue: 100,
      minValue: 0,
    });
    chart.addTimeSeries(series, { strokeStyle: `rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255))`, lineWidth: 2 });
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
