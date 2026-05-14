<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let entries: { name: string; path: string; kind: string }[] = [];
  export let visible = false;

  const dispatch = createEventDispatcher();

  let query = '';

  $: filtered = query.trim()
    ? entries.filter(e => e.name.toLowerCase().includes(query.toLowerCase()))
    : entries;

  function select(entry: typeof entries[number]) {
    visible = false;
    query = '';
    dispatch('select', entry);
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Escape') { visible = false; query = ''; }
  }
</script>

{#if visible}
  <div class="fuzzy_finder" role="dialog" aria-modal="true">
    <input
      type="text"
      placeholder="Type to search…"
      bind:value={query}
      autofocus
      on:keydown={handleKey}
    />
    <ul class="fuzzy_results">
      {#each filtered.slice(0, 30) as e (e.path)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <li class="fuzzy_item fuzzy_{e.kind}" on:click={() => select(e)}>
          <span class="fuzzy_icon">{e.kind === 'dir' ? '▶' : '·'}</span>
          <span class="fuzzy_name">{e.name}</span>
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .fuzzy_finder {
    position: fixed;
    top: 20%;
    left: 50%;
    transform: translateX(-50%);
    width: 40em;
    max-width: 90vw;
    background: rgba(0,0,0,0.95);
    border: 1px solid rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    z-index: 10001;
    padding: 1em;
  }
  input {
    width: 100%;
    background: transparent;
    border: none;
    border-bottom: 1px solid currentColor;
    color: inherit;
    font-family: inherit;
    font-size: 1em;
    margin-bottom: 0.5em;
  }
  .fuzzy_results {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 20em;
    overflow-y: auto;
  }
  .fuzzy_item {
    padding: 0.2em 0.5em;
    cursor: pointer;
  }
  .fuzzy_item:hover {
    background: rgba(255,255,255,0.1);
  }
</style>
