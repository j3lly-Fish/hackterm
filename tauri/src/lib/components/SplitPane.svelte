<script lang="ts">
  export let ratio = 0.5;
  export let enabled = false;

  let dragging = false;
  let container: HTMLDivElement;

  function onMouseDown() { dragging = true; }

  function onMouseMove(e: MouseEvent) {
    if (!dragging || !container) return;
    const rect = container.getBoundingClientRect();
    ratio = Math.max(0.2, Math.min(0.8, (e.clientX - rect.left) / rect.width));
  }

  function onMouseUp() { dragging = false; }
</script>

{#if enabled}
  <div
    class="split_pane"
    bind:this={container}
    on:mousemove={onMouseMove}
    on:mouseup={onMouseUp}
    on:mouseleave={onMouseUp}
  >
    <div class="split_left" style="width: {ratio * 100}%">
      <slot name="left" />
    </div>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="split_divider" on:mousedown={onMouseDown}></div>
    <div class="split_right" style="width: {(1 - ratio) * 100}%">
      <slot name="right" />
    </div>
  </div>
{:else}
  <slot name="left" />
{/if}

<style>
  .split_pane {
    display: flex;
    width: 100%;
    height: 100%;
  }
  .split_divider {
    width: 4px;
    cursor: col-resize;
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.3);
    flex-shrink: 0;
  }
  .split_divider:hover {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.8);
  }
</style>
