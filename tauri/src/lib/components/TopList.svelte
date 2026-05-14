<script lang="ts">
  import { processes } from '$lib/stores/sysinfo';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;

  $: top = $processes
    .filter(p => !settings.excludeThreadsFromToplist || p.name !== 'kthread')
    .slice(0, 10);

  function fmtMem(bytes: number): string {
    if (bytes > 1e9) return (bytes / 1e9).toFixed(1) + 'G';
    return (bytes / 1e6).toFixed(0) + 'M';
  }
</script>

<div class="mod_toplist">
  <h4 class="mod_title">PROCESSES</h4>
  <table>
    <thead>
      <tr><th>PID</th><th>NAME</th><th>CPU%</th><th>MEM</th></tr>
    </thead>
    <tbody>
      {#each top as p (p.pid)}
        <tr>
          <td>{p.pid}</td>
          <td class="proc_name">{p.name}</td>
          <td>{p.cpu.toFixed(1)}</td>
          <td>{fmtMem(p.memory)}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
