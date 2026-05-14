<script lang="ts">
  import { onMount } from 'svelte';

  const CURRENT_VERSION = '2.3.0';
  let updateAvailable = false;
  let latestVersion = '';

  onMount(async () => {
    try {
      const res = await fetch('https://api.github.com/repos/GitSquared/edex-ui/releases/latest');
      if (res.ok) {
        const data = await res.json();
        latestVersion = data.tag_name?.replace(/^v/, '') ?? '';
        // Simple semver comparison
        updateAvailable = latestVersion > CURRENT_VERSION;
      }
    } catch {}
  });
</script>

{#if updateAvailable}
  <div class="update_banner">
    Update available: v{latestVersion}
  </div>
{/if}

<style>
  .update_banner {
    position: fixed;
    bottom: 1em;
    right: 1em;
    background: rgba(0,0,0,0.8);
    border: 1px solid var(--color_yellow, yellow);
    padding: 0.4em 0.8em;
    font-size: 0.8em;
    z-index: 9990;
  }
</style>
