<script lang="ts">
  let src = '';
  let type: 'audio' | 'video' = 'audio';
  let visible = false;

  export function open(fileSrc: string, fileType: 'audio' | 'video' = 'audio') {
    src = fileSrc;
    type = fileType;
    visible = true;
  }

  export function close() {
    visible = false;
    src = '';
  }
</script>

{#if visible}
  <div class="media_player">
    <button class="mp_close" on:click={close}>✕</button>
    {#if type === 'audio'}
      <!-- svelte-ignore a11y-media-has-caption -->
      <audio controls src={src} autoplay></audio>
    {:else}
      <!-- svelte-ignore a11y-media-has-caption -->
      <video controls src={src} autoplay></video>
    {/if}
  </div>
{/if}

<style>
  .media_player {
    position: fixed;
    bottom: 2em;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(0,0,0,0.9);
    border: 1px solid rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    padding: 1em;
    z-index: 9998;
  }
</style>
