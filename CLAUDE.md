# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **Note:** This project was archived on October 18, 2021. No new features or releases are planned.

## Commands

**Install and run from source (Linux/macOS):**
```sh
npm run install-linux   # installs deps in both root and src/, rebuilds node-pty
npm run start           # launches via electron
```

**Install and run from source (Windows, run as admin):**
```sh
npm run install-windows
npm run start
```

**Build distributable (host OS only — native modules prevent cross-compilation):**
```sh
npm install             # NOT install-linux/windows
npm run build-linux     # or build-darwin / build-windows
# Output goes to dist/
```

The build pipeline (`prebuild-*` scripts) copies `src/` to `prebuild-src/`, runs `prebuild-minify.js` to minify JS/CSS/JSON, installs deps, then electron-builder packages it.

## Architecture

eDEX-UI is an Electron app with three concurrent processes:

### Main process (`src/_boot.js`)
- Creates the `BrowserWindow` and loads `src/ui.html`
- Spawns a `Terminal` (node-pty server) on port 3000 (configurable via `settings.json`)
- Supports up to 4 additional TTY tabs, each on port+2 through port+6
- Handles IPC for TTY spawning (`ttyspawn`), theme/keyboard hot-switch, and log forwarding
- Mirrors internal themes, keyboard layouts, and fonts into the OS userData directory on every launch

### Multithreading worker (`src/_multithread.js`)
- Required by `_boot.js`; runs both as cluster master and worker
- Master receives `systeminformation-call` IPC messages from the renderer and round-robins them to worker processes
- Workers execute `systeminformation` calls and return results; capped at 7 workers

### Renderer process (`src/_renderer.js` + `src/ui.html`)
- Loaded inside the BrowserWindow; has Node.js integration enabled
- Reads `settings.json`, `shortcuts.json`, and `lastWindowState.json` from userData on startup
- Applies theme by injecting CSS variables and loading WOFF2 fonts from userData/fonts/
- Exposes `window.si` as a Proxy that routes all `systeminformation` calls through IPC to the multithread controller
- UI components are ES6 classes in `src/classes/` — instantiated in `_renderer.js`; key ones: `Terminal` (client-side xterm.js), `Filesystem`, `CPUInfo`, `RAMWatcher`, `NetStat`, `Keyboard`, `LocationGlobe`, `TopList`, `Clock`, `Modal`

### Terminal I/O
`src/classes/terminal.class.js` is dual-role:
- **Server role** (main process): wraps `node-pty`, exposes a WebSocket server
- **Client role** (renderer): wraps `xterm.js` with `AttachAddon`/`FitAddon`/`WebglAddon`, connects to the WebSocket server at `ws://localhost:<port>`

Terminal resize is encoded as a 6-character string (`cols` + `rows`, each zero-padded to 3 digits) sent over IPC.

## Configuration

User config lives in the OS `userData` directory (e.g., `~/.config/eDEX-UI/` on Linux). Key files:
- `settings.json` — shell, theme, keyboard, port, display options (created with defaults on first run)
- `shortcuts.json` — keyboard shortcut bindings
- `lastWindowState.json` — fullscreen vs windowed

Internal defaults are in `src/assets/` and overwritten into userData on every launch.

## Themes and Keyboard Layouts

Themes are JSON files in `src/assets/themes/`. Schema:
```json
{
  "colors": { "r", "g", "b", "black", "light_black", "grey", "red", "yellow" },
  "cssvars": { "font_main", "font_main_light" },
  "terminal": { "fontFamily", "cursorStyle", "foreground", "background", "cursor", "cursorAccent", "selection", "colorFilter" },
  "globe": { "base", "marker", "pin", "satellite" },
  "injectCSS": ""
}
```

`colorFilter` (optional array) applies color transformations to the terminal using the `color` library (e.g., `"lighten(0.1)"`).

Keyboard layouts are JSON files in `src/assets/kb_layouts/`. The `file-icons-generator.js` script (with the `file-icons` git submodule) regenerates `src/assets/misc/file-icons-match.js`.
