// See https://svelte.dev/docs/kit/types#app.d.ts

declare namespace App {}

// Allow augmented-ui and other non-standard HTML attributes globally
declare namespace svelteHTML {
  interface HTMLAttributes<T> {
    'augmented-ui'?: string;
  }
}
