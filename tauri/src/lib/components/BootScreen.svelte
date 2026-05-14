<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import type { Theme } from '$lib/stores/theme';

  export let theme: Theme;
  export let version = '2.3.0';
  export let nointro = false;

  const dispatch = createEventDispatcher<{ complete: void }>();

  let lines: string[] = [];
  let phase: 'boot' | 'title' | 'done' = 'boot';
  let glitchActive = false;
  let themeColor = '';

  $: themeColor = `rgb(${theme.colors.r}, ${theme.colors.g}, ${theme.colors.b})`;

  async function delay(ms: number) {
    return new Promise(r => setTimeout(r, ms));
  }

  async function runBootLog() {
    try {
      const resp = await fetch('/assets/misc/boot_log.txt');
      const text = await resp.text();
      const log = text.split('\n');

      for (let i = 0; i < log.length; i++) {
        if (log[i] === 'Boot Complete') {
          // play granted sound (handled by parent)
        }
        lines = [...lines, log[i]];

        let wait = 30;
        if (i < 4) wait = i < 2 ? 500 : 400;
        else if (i === 25) wait = 400;
        else if (i === 42) wait = 300;
        else if (i >= log.length - 2) wait = 300;
        else wait = Math.max(5, Math.pow(1 - i / 1000, 3) * 25);
        await delay(wait);
      }
    } catch {}
  }

  async function runTitleScreen() {
    phase = 'title';
    await delay(400);
    glitchActive = true;
    await delay(500);
    glitchActive = false;
    await delay(1000);
    phase = 'done';
    dispatch('complete');
  }

  onMount(() => {
    if (nointro) {
      dispatch('complete');
      return;
    }
    runBootLog().then(runTitleScreen);
  });
</script>

{#if phase === 'boot'}
  <section id="boot_screen" class="solid_background">
    {#each lines as line}
      <span>{line}<br/></span>
    {/each}
  </section>
{:else if phase === 'title'}
  <section id="boot_screen" class="center">
    <h1 class:glitch={glitchActive} style="border: 5px solid {themeColor}">eDEX-UI</h1>
  </section>
{/if}

<style>
  #boot_screen {
    position: fixed;
    inset: 0;
    z-index: 99999;
    font-family: var(--font_mono, monospace);
    font-size: 0.85em;
    padding: 1em;
    background: #000;
    overflow: hidden;
  }
  #boot_screen.center {
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
  }
  h1 {
    font-size: 4em;
    padding: 0.3em 0.6em;
    letter-spacing: 0.2em;
  }
  .glitch {
    animation: glitch 0.3s steps(2) infinite;
  }
  @keyframes glitch {
    0%   { clip-path: inset(10% 0 85% 0); transform: translate(-2px, 0); }
    50%  { clip-path: inset(40% 0 20% 0); transform: translate(2px, 0); }
    100% { clip-path: inset(70% 0 5% 0);  transform: translate(-1px, 0); }
  }
</style>
