import { derived, writable } from 'svelte/store';
import { settings, configDir } from './settings';

export interface ThemeColors {
  r: number;
  g: number;
  b: number;
  black: string;
  light_black: string;
  grey: string;
  red?: string;
  yellow?: string;
}

export interface ThemeCssVars {
  font_main: string;
  font_main_light: string;
}

export interface ThemeTerminal {
  fontFamily: string;
  cursorStyle: string;
  foreground: string;
  background: string;
  cursor: string;
  cursorAccent: string;
  selection: string;
  colorFilter?: string[];
}

export interface ThemeGlobe {
  base: string;
  marker: string;
  pin: string;
  satellite: string;
}

export interface Theme {
  colors: ThemeColors;
  cssvars: ThemeCssVars;
  terminal: ThemeTerminal;
  globe: ThemeGlobe;
  injectCSS?: string;
}

export const currentTheme = writable<Theme | null>(null);

/** Load a theme JSON from the config dir or bundled assets */
export async function loadTheme(themeName: string, dir: string): Promise<Theme | null> {
  try {
    // Try user's config dir first (allows customisation)
    const res = await fetch(`asset://localhost/${encodeURIComponent(dir)}/themes/${themeName}.json`);
    if (res.ok) {
      return await res.json() as Theme;
    }
  } catch {}

  // Fall back to bundled asset
  try {
    const res = await fetch(`/assets/themes/${themeName}.json`);
    if (res.ok) return await res.json() as Theme;
  } catch {}

  return null;
}

/** Apply a theme: inject CSS variables, load fonts */
export function applyTheme(theme: Theme, nocursor = false): void {
  // Remove any existing theming style
  document.querySelector('style.theming')?.remove();

  const purify = (s: string | undefined) => (s ?? '').replace(/[<]/g, '');
  const style = document.createElement('style');
  style.className = 'theming';
  style.textContent = `
:root {
  --font_main: "${purify(theme.cssvars.font_main)}";
  --font_main_light: "${purify(theme.cssvars.font_main_light)}";
  --font_mono: "${purify(theme.terminal.fontFamily)}";
  --color_r: ${purify(String(theme.colors.r))};
  --color_g: ${purify(String(theme.colors.g))};
  --color_b: ${purify(String(theme.colors.b))};
  --color_black: ${purify(theme.colors.black)};
  --color_light_black: ${purify(theme.colors.light_black)};
  --color_grey: ${purify(theme.colors.grey)};
  --color_red: ${purify(theme.colors.red ?? 'red')};
  --color_yellow: ${purify(theme.colors.yellow ?? 'yellow')};
}
body {
  font-family: var(--font_main), sans-serif;
  cursor: ${nocursor ? 'none' : 'default'} !important;
}
${nocursor ? '* { cursor: none !important; }' : ''}
${purify(theme.injectCSS ?? '')}
`;
  document.head.appendChild(style);
  currentTheme.set(theme);
}
