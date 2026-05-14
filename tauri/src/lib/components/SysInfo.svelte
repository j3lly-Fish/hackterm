<script lang="ts">
  import { systemInfo } from '$lib/stores/sysinfo';

  function fmtUptime(secs: number): string {
    const d = Math.floor(secs / 86400);
    const h = Math.floor((secs % 86400) / 3600);
    const m = Math.floor((secs % 3600) / 60);
    const s = secs % 60;
    if (d > 0) return `${d}d ${h}h ${m}m`;
    return `${h}h ${m}m ${s}s`;
  }
</script>

<div class="mod_sysinfo">
  <h4 class="mod_title">SYSTEM</h4>
  {#if $systemInfo}
    <p><span class="label">HOST</span>{$systemInfo.hostname}</p>
    <p><span class="label">OS</span>{$systemInfo.os_name} {$systemInfo.os_version}</p>
    <p><span class="label">KERNEL</span>{$systemInfo.kernel_version}</p>
    <p><span class="label">UPTIME</span>{fmtUptime($systemInfo.uptime)}</p>
  {:else}
    <p>...</p>
  {/if}
</div>
