<script lang="ts">
  export let type: 'info' | 'error' | 'warning' | 'confirm' = 'info';
  export let title = '';
  export let message = '';
  export let visible = false;
  export let onClose: (() => void) | null = null;
  export let onConfirm: (() => void) | null = null;

  function close() {
    visible = false;
    onClose?.();
  }

  function confirm() {
    visible = false;
    onConfirm?.();
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="modal_overlay" on:click|self={close}>
    <div class="modal mod_{type}" augmented-ui="tl-clip br-clip exe">
      <h3 class="modal_title">{title}</h3>
      <div class="modal_body">
        {@html message}
      </div>
      <div class="modal_actions">
        {#if type === 'confirm'}
          <button class="modal_btn modal_confirm" on:click={confirm}>CONFIRM</button>
          <button class="modal_btn modal_cancel" on:click={close}>CANCEL</button>
        {:else}
          <button class="modal_btn" on:click={close}>CLOSE</button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .modal_overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.7);
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .modal {
    min-width: 320px;
    max-width: 600px;
    padding: 1.5em;
    background: rgba(0,0,0,0.9);
    border: 1px solid rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
  }
</style>
