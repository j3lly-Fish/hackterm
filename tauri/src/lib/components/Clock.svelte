<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;

  let timeStr = '';
  let dateStr = '';
  let timerId: ReturnType<typeof setInterval>;

  function update() {
    const now = new Date();
    const h = settings.clockHours === 12
      ? now.getHours() % 12 || 12
      : now.getHours();
    const hh = String(h).padStart(2, '0');
    const mm = String(now.getMinutes()).padStart(2, '0');
    const ss = String(now.getSeconds()).padStart(2, '0');
    timeStr = `${hh}:${mm}:${ss}`;
    dateStr = now.toLocaleDateString('en-US', { weekday: 'short', year: 'numeric', month: 'short', day: 'numeric' });
  }

  onMount(() => {
    update();
    timerId = setInterval(update, 1000);
  });

  onDestroy(() => clearInterval(timerId));
</script>

<div class="mod_clock">
  <p class="time">{timeStr}</p>
  <p class="date">{dateStr}</p>
</div>
