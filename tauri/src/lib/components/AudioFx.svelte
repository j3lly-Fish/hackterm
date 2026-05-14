<script lang="ts">
  import { onMount } from 'svelte';
  import type { AppSettings } from '$lib/stores/settings';

  export let settings: AppSettings;

  type SoundName = 'alarm' | 'denied' | 'error' | 'expand' | 'folder' | 'granted'
    | 'info' | 'keyboard' | 'panels' | 'scan' | 'stdin' | 'stdout' | 'theme';

  let sounds: Partial<Record<SoundName, any>> = {};

  export function play(name: SoundName) {
    if (!settings.audio || settings.disableFeedbackAudio) return;
    sounds[name]?.play();
  }

  export function playAlarm() {
    if (!settings.audio) return;
    sounds.alarm?.play();
  }

  onMount(async () => {
    const { Howl } = await import('howler');
    const names: SoundName[] = [
      'alarm', 'denied', 'error', 'expand', 'folder', 'granted',
      'info', 'keyboard', 'panels', 'scan', 'stdin', 'stdout', 'theme',
    ];

    for (const name of names) {
      sounds[name] = new Howl({
        src: [`/assets/audio/${name}.wav`],
        volume: settings.audioVolume,
        html5: true,
        onloaderror: () => { /* silently ignore missing/unsupported audio */ },
        onplayerror: () => { /* silently ignore playback failures */ },
      });
    }
  });
</script>
