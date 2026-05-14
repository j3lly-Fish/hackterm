<script lang="ts">
  import type { SshProfile } from '$lib/stores/settings';

  export let profiles: SshProfile[] = [];
  export let onConnect: (host: string, port: number, username: string, password: string, wsPort: number) => void;
  export let onClose: () => void;

  let host = '';
  let port = 22;
  let username = '';
  let password = '';
  let wsPort = 3010;
  let selectedProfile = '';

  function selectProfile(p: SshProfile) {
    host = p.host;
    port = p.port;
    username = p.username;
    selectedProfile = p.name;
    password = '';
  }

  function connect() {
    if (!host || !username) return;
    onConnect(host, port, username, password, wsPort);
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter') connect();
    if (e.key === 'Escape') onClose();
  }
</script>

<svelte:window on:keydown={handleKey} />

<div class="ssh_overlay" on:click|self={onClose} role="dialog" aria-label="SSH Connect">
  <div class="ssh_dialog" augmented-ui="tr-clip bl-clip exe">
    <header>
      <h2>SSH CONNECT</h2>
      <button class="close_btn" on:click={onClose}>×</button>
    </header>

    {#if profiles.length > 0}
      <div class="profiles">
        {#each profiles as p}
          <button class:active={selectedProfile === p.name} on:click={() => selectProfile(p)}>
            {p.name}
          </button>
        {/each}
      </div>
    {/if}

    <div class="form">
      <label class="field">
        <span>HOST</span>
        <input type="text" bind:value={host} placeholder="hostname or IP" />
      </label>
      <label class="field">
        <span>PORT</span>
        <input type="number" bind:value={port} min="1" max="65535" />
      </label>
      <label class="field">
        <span>USERNAME</span>
        <input type="text" bind:value={username} />
      </label>
      <label class="field">
        <span>PASSWORD</span>
        <input type="password" bind:value={password} />
      </label>
    </div>

    <footer>
      <button class="btn_connect" on:click={connect} disabled={!host || !username}>CONNECT</button>
      <button on:click={onClose}>CANCEL</button>
    </footer>
  </div>
</div>

<style>
  .ssh_overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 900;
  }
  .ssh_dialog {
    width: 400px;
    background: rgba(0, 0, 0, 0.92);
    border: 1px solid rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
    font-family: var(--font_main, monospace);
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 16px;
    border-bottom: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.3);
  }
  header h2 { margin: 0; font-size: 0.9em; letter-spacing: 0.25em; }
  .close_btn {
    background: none;
    border: none;
    color: inherit;
    font-size: 1.4em;
    cursor: pointer;
    padding: 0 4px;
  }
  .profiles {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    padding: 8px 16px;
    border-bottom: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.2);
  }
  .profiles button {
    background: none;
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.4);
    color: inherit;
    font-family: inherit;
    font-size: 0.72em;
    padding: 4px 12px;
    cursor: pointer;
    letter-spacing: 0.06em;
  }
  .profiles button.active {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.18);
    border-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255));
  }
  .form {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .field span {
    font-size: 0.68em;
    letter-spacing: 0.12em;
    opacity: 0.7;
  }
  input {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.08);
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.35);
    color: inherit;
    font-family: inherit;
    font-size: 0.85em;
    padding: 5px 8px;
    outline: none;
  }
  footer {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    padding: 10px 16px;
    border-top: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.3);
  }
  footer button {
    background: none;
    border: 1px solid rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.6);
    color: inherit;
    font-family: inherit;
    font-size: 0.72em;
    padding: 6px 22px;
    cursor: pointer;
    letter-spacing: 0.12em;
  }
  footer button:disabled { opacity: 0.35; cursor: default; }
  .btn_connect {
    background: rgba(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255), 0.15) !important;
    border-color: rgb(var(--color_r, 0), var(--color_g, 200), var(--color_b, 255)) !important;
  }
</style>
