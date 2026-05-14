<script lang="ts">
  import { onMount } from 'svelte';
  import { cpuLoad, cpuTemps, memory } from '$lib/stores/sysinfo';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;
  export let playAlert: () => void = () => {};

  interface Alert {
    id: number;
    message: string;
    kind: 'cpu' | 'ram' | 'temp';
    time: number;
  }

  let alerts: Alert[] = [];
  let idCounter = 0;
  const cooldowns = new Map<string, number>();

  function maybeAlert(kind: 'cpu' | 'ram' | 'temp', message: string) {
    if (!settings.alerts.enabled) return;
    const now = Date.now();
    const last = cooldowns.get(kind) ?? 0;
    if (now - last < settings.alerts.cooldownSeconds * 1000) return;
    cooldowns.set(kind, now);
    alerts = [...alerts, { id: ++idCounter, message, kind, time: now }];
    playAlert();
    setTimeout(() => {
      alerts = alerts.filter(a => a.id !== idCounter);
    }, 8000);
  }

  $: {
    const { cpuThreshold } = settings.alerts;
    if ($cpuLoad.global > cpuThreshold)
      maybeAlert('cpu', `CPU usage above ${cpuThreshold}% (${$cpuLoad.global.toFixed(0)}%)`);
  }

  $: {
    const { ramThreshold } = settings.alerts;
    if ($memory && $memory.total > 0) {
      const pct = ($memory.used / $memory.total) * 100;
      if (pct > ramThreshold)
        maybeAlert('ram', `RAM usage above ${ramThreshold}% (${pct.toFixed(0)}%)`);
    }
  }

  $: {
    const { tempThreshold } = settings.alerts;
    if ($cpuTemps.some(t => t.temperature > tempThreshold))
      maybeAlert('temp', `CPU temperature above ${tempThreshold}°C`);
  }
</script>

<div class="alert_manager">
  {#each alerts as a (a.id)}
    <div class="alert alert_{a.kind}">{a.message}</div>
  {/each}
</div>

<style>
  .alert_manager {
    position: fixed;
    top: 1em;
    right: 1em;
    z-index: 10000;
    display: flex;
    flex-direction: column;
    gap: 0.5em;
    pointer-events: none;
  }
  .alert {
    padding: 0.5em 1em;
    background: rgba(0,0,0,0.85);
    border-left: 3px solid var(--color_red, red);
    font-size: 0.85em;
    animation: fadeIn 0.3s ease;
  }
  @keyframes fadeIn {
    from { opacity: 0; transform: translateX(20px); }
    to { opacity: 1; transform: translateX(0); }
  }
</style>
