<script lang="ts">
  import { onMount } from 'svelte';

  export let src = '';
  export let visible = false;

  let canvas: HTMLCanvasElement;
  let currentPage = 1;
  let totalPages = 0;
  let pdfDoc: any = null;

  async function loadPdf(url: string) {
    const pdfjsLib = (window as any).pdfjsLib;
    if (!pdfjsLib) return;
    pdfDoc = await pdfjsLib.getDocument(url).promise;
    totalPages = pdfDoc.numPages;
    renderPage(1);
  }

  async function renderPage(num: number) {
    if (!pdfDoc || !canvas) return;
    const page = await pdfDoc.getPage(num);
    const scale = 1.5;
    const viewport = page.getViewport({ scale });
    canvas.height = viewport.height;
    canvas.width = viewport.width;
    await page.render({ canvasContext: canvas.getContext('2d')!, viewport }).promise;
    currentPage = num;
  }

  onMount(async () => {
    // Load PDF.js dynamically
    if (!(window as any).pdfjsLib) {
      await new Promise<void>((resolve) => {
        const s = document.createElement('script');
        s.src = '/node_modules/pdfjs-dist/build/pdf.min.mjs';
        s.type = 'module';
        s.onload = () => resolve();
        document.head.appendChild(s);
      });
    }
    if (src) loadPdf(src);
  });

  $: if (src && visible) loadPdf(src);
</script>

{#if visible}
  <div class="doc_reader">
    <div class="doc_controls">
      <button disabled={currentPage <= 1} on:click={() => renderPage(currentPage - 1)}>◀</button>
      <span>{currentPage} / {totalPages}</span>
      <button disabled={currentPage >= totalPages} on:click={() => renderPage(currentPage + 1)}>▶</button>
    </div>
    <canvas bind:this={canvas}></canvas>
  </div>
{/if}

<style>
  .doc_reader {
    position: fixed;
    inset: 2em;
    background: rgba(0,0,0,0.95);
    border: 1px solid rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    z-index: 9997;
    overflow: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
