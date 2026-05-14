<script lang="ts">
  import { onMount } from 'svelte';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;
  export let onKeyPress: (cmd: string) => void = () => {};

  interface KeyDef {
    name?: string;
    cmd?: string;
    shift_name?: string;
    shift_cmd?: string;
    ctrl_cmd?: string;
    alt_cmd?: string;
    fn_cmd?: string;
    width?: number;
  }

  interface KbLayout {
    row_numbers?: KeyDef[];
    row_1?: KeyDef[];
    row_2?: KeyDef[];
    row_3?: KeyDef[];
    row_space?: KeyDef[];
  }

  let layout: KbLayout | null = null;
  let shiftActive = false;
  let ctrlActive = false;
  let altActive = false;
  let fnActive = false;

  onMount(async () => {
    const name = settings.keyboard;
    try {
      const res = await fetch(`/assets/kb_layouts/${name}.json`);
      layout = await res.json();
    } catch {
      console.error('Failed to load keyboard layout:', name);
    }
  });

  function handleKey(key: KeyDef) {
    let cmd = '';
    if (ctrlActive && key.ctrl_cmd) cmd = key.ctrl_cmd;
    else if (altActive && key.alt_cmd) cmd = key.alt_cmd;
    else if (fnActive && key.fn_cmd) cmd = key.fn_cmd;
    else if (shiftActive && key.shift_cmd) cmd = key.shift_cmd;
    else cmd = key.cmd ?? '';

    if (!cmd) return;

    // Handle special control sequences
    if (cmd.includes('~~~CTRLSEQ')) {
      // Map to actual terminal escape codes if needed
    }

    onKeyPress(cmd);
    shiftActive = false; // auto-release shift after key
  }

  function renderRows(rows: KeyDef[]): any[] {
    return rows;
  }
</script>

<div id="keyboard">
  {#if layout}
    {#each ['row_numbers', 'row_1', 'row_2', 'row_3', 'row_space'] as rowKey}
      {#if layout[rowKey as keyof KbLayout]}
        <div class="kb_row kb_{rowKey}">
          {#each (layout[rowKey as keyof KbLayout] ?? []) as key}
            {#if key.name === 'SHIFT'}
              <button
                class="kb_key kb_shift"
                class:active={shiftActive}
                on:click={() => (shiftActive = !shiftActive)}
              >{key.name}</button>
            {:else if key.name === 'CTRL'}
              <button
                class="kb_key kb_ctrl"
                class:active={ctrlActive}
                on:click={() => (ctrlActive = !ctrlActive)}
              >{key.name}</button>
            {:else if key.name === 'ALT'}
              <button
                class="kb_key kb_alt"
                class:active={altActive}
                on:click={() => (altActive = !altActive)}
              >{key.name}</button>
            {:else if key.name === 'FN'}
              <button
                class="kb_key kb_fn"
                class:active={fnActive}
                on:click={() => (fnActive = !fnActive)}
              >{key.name}</button>
            {:else}
              <button
                class="kb_key"
                style={key.width ? `flex: ${key.width}` : ''}
                on:click={() => handleKey(key)}
              >
                <span class="kb_main">{shiftActive ? (key.shift_name ?? key.name ?? '') : (key.name ?? '')}</span>
              </button>
            {/if}
          {/each}
        </div>
      {/if}
    {/each}
  {:else}
    <p>Loading keyboard…</p>
  {/if}
</div>
