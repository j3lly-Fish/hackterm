<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Theme } from '$lib/stores/theme';

  export let theme: Theme;

  let container: HTMLDivElement;
  let globe: any = null;

  onMount(async () => {
    // Dynamically load the vendor globe script
    await new Promise<void>((resolve, reject) => {
      const script = document.createElement('script');
      script.src = '/assets/vendor/encom-globe.js';
      script.onload = () => resolve();
      script.onerror = reject;
      document.head.appendChild(script);
    });

    const GridData = await fetch('/assets/misc/grid.json').then(r => r.json()).catch(() => []);

    const EncomGlobe = (window as any).EncomGlobe;
    if (!EncomGlobe) return;

    globe = new EncomGlobe(container, GridData, {
      baseColor: theme.globe.base,
      markerColor: theme.globe.marker,
      pinColor: theme.globe.pin,
      satelliteColor: theme.globe.satellite,
    });
    globe.play();

    // Get geolocation and add marker
    try {
      const ip = await invoke<string>('get_external_ip');
      const geo = await invoke<any>('get_geolocation', { ip });
      if (geo?.latitude && geo?.longitude) {
        globe.addMarker(geo.latitude, geo.longitude, 0.02);
      }
    } catch {}
  });

  onDestroy(() => { globe?.stop(); });
</script>

<div class="mod_globe">
  <h4 class="mod_title">LOCATION</h4>
  <div bind:this={container} class="globe_container"></div>
</div>

<style>
  .globe_container {
    width: 100%;
    aspect-ratio: 1;
  }
</style>
