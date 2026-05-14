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

  const ICONS: Record<string, string> = {
    ARROW_UP: `<svg viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" fill-opacity="1" d="m12 8 5 5h-3v4H10v-4H7z"/><path stroke-linejoin="round" fill="currentColor" fill-opacity="0.45" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    ARROW_DOWN: `<svg viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" fill-opacity="1" d="m12 17-5-5h3V8h4v4h3z"/><path stroke-linejoin="round" fill="currentColor" fill-opacity="0.45" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    ARROW_LEFT: `<svg viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" fill-opacity="1" d="m8 12 5-5v3h4v4h-4v3z"/><path stroke-linejoin="round" fill="currentColor" fill-opacity="0.45" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    ARROW_RIGHT: `<svg viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" fill-opacity="1" d="m16 12-5 5v-3H7v-4h4V7z"/><path stroke-linejoin="round" fill="currentColor" fill-opacity="0.45" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    CLOSE: `<svg viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" fill-opacity="1" d="M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>`,
  };

  // Replace ~~~CTRLSEQn~~~ with the actual escape byte (only index 1 = ESC is used in layouts)
  function expandCtrlSeqs(cmd: string): string {
    return cmd.replace(/~~~CTRLSEQ(\d+)~~~/g, (_m, n) => {
      const idx = parseInt(n, 10);
      if (idx === 1) return '\x1b';
      return '';
    });
  }

  // Returns {html, isIcon} for a key name
  function renderLabel(name: string | undefined): { html: string; isIcon: boolean } {
    if (!name) return { html: '', isIcon: false };
    const m = name.match(/^ESCAPED\|-- ICON: (\w+)$/);
    if (m) {
      const svg = ICONS[m[1]];
      return { html: svg ?? m[1], isIcon: true };
    }
    return { html: name, isIcon: false };
  }

  let layout: KbLayout | null = null;
  let shiftActive = false;
  let ctrlActive = false;
  let altActive = false;
  let fnActive = false;
  let capsActive = false;

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
    let raw = '';
    if (ctrlActive && key.ctrl_cmd) raw = key.ctrl_cmd;
    else if (altActive && key.alt_cmd) raw = key.alt_cmd;
    else if (fnActive && key.fn_cmd) raw = key.fn_cmd;
    else if (shiftActive && key.shift_cmd) raw = key.shift_cmd;
    else raw = key.cmd ?? key.name ?? '';

    if (!raw) return;

    // Handle ESCAPED|-- modifier/special commands
    if (raw.startsWith('ESCAPED|-- ')) {
      const inner = raw.slice('ESCAPED|-- '.length);
      if (inner.startsWith('SHIFT:')) { shiftActive = !shiftActive; return; }
      if (inner.startsWith('CTRL:')) { ctrlActive = !ctrlActive; return; }
      if (inner.startsWith('ALT:')) { altActive = !altActive; return; }
      if (inner.startsWith('FN:')) { fnActive = !fnActive; return; }
      if (inner.startsWith('CAPSLCK:')) { capsActive = !capsActive; return; }
      // ICON keys fall through — their cmd is the actual sequence
      // (name is ESCAPED|-- ICON: X but cmd is the terminal sequence)
      return;
    }

    const cmd = expandCtrlSeqs(raw);
    onKeyPress(cmd);

    // Auto-release one-shot modifiers
    if (!key.name?.startsWith('SHIFT') && !key.name?.startsWith('CTRL') &&
        !key.name?.startsWith('ALT') && !key.name?.startsWith('FN')) {
      shiftActive = false;
      ctrlActive = false;
      altActive = false;
    }
  }

  // Arrow/icon keys have name = "ESCAPED|-- ICON: X" and cmd = the actual sequence
  function handleIconKey(key: KeyDef) {
    const raw = key.cmd ?? '';
    if (!raw) return;
    const cmd = expandCtrlSeqs(raw);
    onKeyPress(cmd);
  }
</script>

<div id="keyboard">
  {#if layout}
    {#each ['row_numbers', 'row_1', 'row_2', 'row_3', 'row_space'] as rowKey}
      {#if layout[rowKey as keyof KbLayout]}
        <div class="kb_row kb_{rowKey}">
          {#each (layout[rowKey as keyof KbLayout] ?? []) as key}
            {@const nameStr = key.name ?? ''}
            {@const isIconKey = nameStr.startsWith('ESCAPED|-- ICON:')}
            {@const isModifier = nameStr === 'SHIFT' || nameStr === 'CTRL' || nameStr === 'ALT' || nameStr === 'FN' || nameStr === 'CAPS'}
            {@const isCmdModifier = (key.cmd ?? '').startsWith('ESCAPED|-- ')}

            {#if isIconKey}
              {@const iconMatch = nameStr.match(/ICON: (\w+)/)}
              {@const iconSvg = iconMatch ? (ICONS[iconMatch[1]] ?? iconMatch[1]) : nameStr}
              <button
                class="kb_key kb_icon"
                style={key.width ? `flex: ${key.width}` : ''}
                on:click={() => handleIconKey(key)}
              >{@html iconSvg}</button>
            {:else if isCmdModifier}
              {@const modMatch = (key.cmd ?? '').match(/ESCAPED\|-- (\w+):/)}
              {@const modName = modMatch?.[1]?.toLowerCase() ?? 'mod'}
              <button
                class="kb_key kb_{modName}"
                class:active={
                  modName === 'shift' ? shiftActive :
                  modName === 'ctrl' ? ctrlActive :
                  modName === 'alt' ? altActive :
                  modName === 'fn' ? fnActive :
                  modName === 'capslck' ? capsActive : false
                }
                style={key.width ? `flex: ${key.width}` : ''}
                on:click={() => handleKey(key)}
              >{key.name ?? modName.toUpperCase()}</button>
            {:else}
              <button
                class="kb_key"
                style={key.width ? `flex: ${key.width}` : ''}
                on:click={() => handleKey(key)}
              >
                <span class="kb_main">
                  {shiftActive || capsActive
                    ? (key.shift_name ?? key.name ?? '')
                    : (key.name ?? '')}
                </span>
                {#if key.shift_name && key.shift_name !== key.name}
                  <span class="kb_shift_label">{key.shift_name}</span>
                {/if}
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
