<script lang="ts">
  import { onMount } from 'svelte';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;

  type SoundName = 'alarm' | 'denied' | 'error' | 'expand' | 'folder' | 'granted'
    | 'info' | 'keyboard' | 'panels' | 'scan' | 'stdin' | 'stdout' | 'theme';

  let sounds: Partial<Record<SoundName, any>> = {};
  let audioAvailable = false;

  export function play(name: SoundName) {
    if (!audioAvailable || !settings.audio || settings.disableFeedbackAudio) return;
    try { sounds[name]?.play(); } catch {}
  }

  export function playAlarm() {
    if (!audioAvailable || !settings.audio) return;
    try { sounds.alarm?.play(); } catch {}
  }

  onMount(async () => {
    if (!settings.audio) return;

    try {
      // Patch AudioContext before Howler imports it so the global ctx creation
      // doesn't throw an unhandled error on Linux/WebKit where audio may be unavailable.
      const OrigAudioContext = (window as any).AudioContext || (window as any).webkitAudioContext;
      if (OrigAudioContext) {
        (window as any)._AudioContextSafe = function(...args: any[]) {
          try { return new OrigAudioContext(...args); } catch { return null; }
        };
        (window as any)._AudioContextSafe.prototype = OrigAudioContext.prototype;
      }

      const { Howl, Howler } = await import('howler');

      // Disable the Howler global suspend loop — it can throw on broken audio backends
      try { Howler.autoSuspend = false; } catch {}

      const names: SoundName[] = [
        'alarm', 'denied', 'error', 'expand', 'folder', 'granted',
        'info', 'keyboard', 'panels', 'scan', 'stdin', 'stdout', 'theme',
      ];

      for (const name of names) {
        try {
          sounds[name] = new Howl({
            src: [`/assets/audio/${name}.wav`],
            volume: settings.audioVolume ?? 0.5,
            html5: true,
            onloaderror: () => {},
            onplayerror: () => {},
          });
        } catch {}
      }

      audioAvailable = true;
    } catch {
      // Audio backend unavailable (common on Linux/WebKit) — degrade silently
    }
  });
</script>
