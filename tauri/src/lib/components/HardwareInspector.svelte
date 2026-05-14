<script lang="ts">
  import { hardwareInfo } from '$lib/stores/sysinfo';

  function fmtBytes(b: number): string {
    return (b / 1e9).toFixed(1) + ' GB';
  }
</script>

<div class="mod_hardwareInspector">
  <h4 class="mod_title">HARDWARE</h4>
  {#if $hardwareInfo}
    <p><span class="label">CPU</span>{$hardwareInfo.cpu_brand}</p>
    <p><span class="label">CORES</span>{$hardwareInfo.cpu_cores}</p>
    <p><span class="label">RAM</span>{fmtBytes($hardwareInfo.total_memory)}</p>
    {#each $hardwareInfo.disks as d}
      <p><span class="label">DISK</span>{d.mount} {fmtBytes(d.total)}</p>
    {/each}
  {:else}
    <p>...</p>
  {/if}
</div>
