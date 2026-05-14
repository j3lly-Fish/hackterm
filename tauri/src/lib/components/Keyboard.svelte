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
    ARROW_UP:    `<svg viewBox="0 0 24 24"><path fill="currentColor" d="m12 8 5 5h-3v4H10v-4H7z"/><path fill="currentColor" fill-opacity=".35" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    ARROW_DOWN:  `<svg viewBox="0 0 24 24"><path fill="currentColor" d="m12 17-5-5h3V8h4v4h3z"/><path fill="currentColor" fill-opacity=".35" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    ARROW_LEFT:  `<svg viewBox="0 0 24 24"><path fill="currentColor" d="m8 12 5-5v3h4v4h-4v3z"/><path fill="currentColor" fill-opacity=".35" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    ARROW_RIGHT: `<svg viewBox="0 0 24 24"><path fill="currentColor" d="m16 12-5 5v-3H7v-4h4V7z"/><path fill="currentColor" fill-opacity=".35" d="M4 3h16c1.1 0 1-.1 1 1v16c0 1.1.1 1-1 1H4c-1.1 0-1 .1-1-1V4c0-1.1-.1-1 1-1zm0 1v16h16V4z"/></svg>`,
    CLOSE:       `<svg viewBox="0 0 24 24"><path fill="currentColor" d="M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>`,
  };

  function expandCtrlSeqs(cmd: string): string {
    return cmd.replace(/~~~CTRLSEQ(\d+)~~~/g, (_m, n) => parseInt(n, 10) === 1 ? '\x1b' : '');
  }

  function iconName(name: string): string | null {
    const m = name.match(/^ESCAPED\|-- ICON: (\w+)$/);
    return m ? m[1] : null;
  }

  function isEscaped(name: string): boolean {
    return name.startsWith('ESCAPED|-- ');
  }

  function modifierKey(cmd: string): string | null {
    const m = cmd?.match(/^ESCAPED\|-- (\w+):/);
    return m ? m[1].toLowerCase() : null;
  }

  let layout: KbLayout | null = null;
  let shiftActive = false;
  let ctrlActive = false;
  let altActive = false;
  let fnActive = false;
  let capsActive = false;

  onMount(async () => {
    try {
      const res = await fetch(`/assets/kb_layouts/${settings.keyboard}.json`);
      layout = await res.json();
    } catch {
      console.error('Failed to load keyboard layout:', settings.keyboard);
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

    if (raw.startsWith('ESCAPED|-- ')) {
      const inner = raw.slice('ESCAPED|-- '.length);
      if (inner.startsWith('SHIFT:'))   { shiftActive = !shiftActive; return; }
      if (inner.startsWith('CTRL:'))    { ctrlActive  = !ctrlActive;  return; }
      if (inner.startsWith('ALT:'))     { altActive   = !altActive;   return; }
      if (inner.startsWith('FN:'))      { fnActive    = !fnActive;    return; }
      if (inner.startsWith('CAPSLCK:')) { capsActive  = !capsActive;  return; }
      return;
    }

    onKeyPress(expandCtrlSeqs(raw));

    if (!isEscaped(key.name ?? '')) {
      shiftActive = false;
      ctrlActive  = false;
      altActive   = false;
    }
  }

  function handleIconKey(key: KeyDef) {
    const raw = key.cmd ?? '';
    if (raw) onKeyPress(expandCtrlSeqs(raw));
  }

  // Determine extra classes for a key
  function keyClass(key: KeyDef): string {
    const n = key.name ?? '';
    if (n === 'ENTER') return 'keyboard_key keyboard_enter';
    if (n.startsWith('SHIFT')) return 'keyboard_key keyboard_shift';
    if (n.startsWith('CTRL') || n.startsWith('CTRL')) return 'keyboard_key keyboard_ctrl';
    if (n === 'CAPS') return 'keyboard_key keyboard_caps';
    if (n === 'FN') return 'keyboard_key keyboard_fn';
    return 'keyboard_key';
  }

  function isActive(key: KeyDef): boolean {
    const mod = modifierKey(key.cmd ?? '');
    if (!mod) return false;
    if (mod === 'shift') return shiftActive;
    if (mod === 'ctrl') return ctrlActive;
    if (mod === 'alt') return altActive;
    if (mod === 'fn') return fnActive;
    if (mod === 'capslck') return capsActive;
    return false;
  }

  const ROWS = ['row_numbers', 'row_1', 'row_2', 'row_3', 'row_space'] as const;
</script>

<section
  id="keyboard"
  data-is-shift-on={shiftActive}
  data-is-caps-lck-on={capsActive}
  data-is-fn-on={fnActive}
>
  {#if layout}
    {#each ROWS as rowKey}
      {#if layout[rowKey as keyof KbLayout]}
        <div class="keyboard_row" id={rowKey}>
          {#each (layout[rowKey as keyof KbLayout] ?? []) as key}
            {@const name = key.name ?? ''}
            {@const icon = iconName(name)}
            {@const isMod = isEscaped(key.cmd ?? '') && !icon}

            {#if icon}
              <!-- Arrow / close keys -->
              <div
                class="keyboard_key"
                style={key.width ? `flex:${key.width}` : ''}
                on:click={() => handleIconKey(key)}
                role="button"
                tabindex="-1"
              >{@html ICONS[icon] ?? icon}</div>
            {:else if name === 'ENTER'}
              <div
                class="keyboard_key keyboard_enter"
                on:click={() => handleKey(key)}
                role="button"
                tabindex="-1"
              ><h1>ENTER</h1></div>
            {:else if name === 'BACK' || name === 'BACK DELETE'}
              <div
                class="keyboard_key keyboard_backspace"
                on:click={() => handleKey(key)}
                role="button"
                tabindex="-1"
              ><h1>{name}</h1></div>
            {:else if isMod}
              <div
                class="keyboard_key"
                class:active={isActive(key)}
                data-cmd={key.cmd}
                style={key.width ? `flex:${key.width}` : ''}
                on:click={() => handleKey(key)}
                role="button"
                tabindex="-1"
              ><h1>{name}</h1></div>
            {:else if name === ' ' || key.cmd === ' '}
              <div
                class="keyboard_key"
                id="keyboard_spacebar"
                on:click={() => handleKey(key)}
                role="button"
                tabindex="-1"
              ></div>
            {:else}
              <div
                class={keyClass(key)}
                style={key.width ? `flex:${key.width}` : ''}
                on:click={() => handleKey(key)}
                role="button"
                tabindex="-1"
              >
                {#if key.shift_name && key.shift_name !== name}
                  <h2>{key.shift_name}</h2>
                {/if}
                <h1>{shiftActive || capsActive ? (key.shift_name ?? name) : name}</h1>
              </div>
            {/if}
          {/each}
        </div>
      {/if}
    {/each}
  {/if}
</section>
