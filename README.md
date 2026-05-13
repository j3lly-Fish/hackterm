<p align="center">
  <br>
  <img alt="Logo" src="media/logo.png">
  <br><br>
  <strong>hackterm</strong>
  <br>
  <em>The Hackers Terminal</em>
  <br><br>
  <a href="https://github.com/j3lly-Fish/hackterm/releases/latest"><img alt="Release" src="https://img.shields.io/github/v/release/j3lly-Fish/hackterm?style=popout"></a>
  <a href="https://github.com/j3lly-Fish/hackterm/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/j3lly-Fish/hackterm?style=popout"></a>
  <br><br>
</p>

hackterm is a fullscreen, cross-platform terminal emulator and system monitor that looks and feels like a sci-fi computer interface.

> **hackterm is a fork of [eDEX-UI](https://github.com/GitSquared/edex-ui)** by [GitSquared](https://github.com/GitSquared). I came across eDEX-UI and thought it was one of the coolest terminal projects I'd ever seen. When the original was archived in 2021, I wanted to make sure development didn't stop there — so I forked it, modernized the internals, and added new features to keep the project alive and growing.

---

<a href="https://youtu.be/BGeY1rK19zA">
  <img align="right" width="400" alt="Demo on YouTube" src="media/youtube-demo-teaser.gif">
</a>

Heavily inspired by the [TRON Legacy movie effects](https://web.archive.org/web/20170511000410/http://jtnimoy.com/blogs/projects/14881671) (especially the [Board Room sequence](https://gmunk.com/TRON-Board-Room)), the project strives to maintain a certain level of functionality and to be usable in real-life scenarios, with the larger goal of bringing science-fiction UXs to the mainstream.

It might or might not be a joke taken too seriously.

---

<p align="center">
  <em>Jump to: <br><a href="#features">Features</a> — <a href="#screenshots">Screenshots</a> — <a href="#qa">Questions & Answers</a> — <a href="#getting-started">Getting Started</a> — <a href="#credits">Credits</a></em>
</p>

## Features

- Fully featured terminal emulator with tabs, colors, mouse events, and support for `curses`-like applications.
- **Split-pane terminals** — view two shells side by side, drag the divider to resize (`Ctrl+Shift+\`).
- **SSH sessions** — connect to remote machines directly from the interface (`Ctrl+Shift+E`); sessions appear as regular terminal tabs.
- **Threshold-based system alerts** — get notified with a modal, audio alarm, and panel flash when CPU, RAM, or CPU temperature exceeds configurable limits.
- **In-app settings editor** — tabbed settings UI covering General, Alerts, Display, and SSH Profiles; no more hand-editing JSON.
- Real-time system (CPU, RAM, swap, processes) and network (GeoIP, active connections, transfer rates) monitoring.
- Full support for touch-enabled displays, including an on-screen keyboard.
- Directory viewer that follows the CWD of the terminal.
- Advanced customization using themes, on-screen keyboard layouts, and CSS injections.
- Optional sound effects for maximum hollywood hacking vibe.

## Screenshots

![Default screenshot](media/screenshot_default.png)

_[neofetch](https://github.com/dylanaraps/neofetch) with the default "tron" theme & QWERTY keyboard_

![Blade screenshot](media/screenshot_blade.png)

_Checking out available themes with [`ranger`](https://github.com/ranger/ranger) on the "blade" theme_

![Disrupted screenshot](media/screenshot_disrupted.png)

_[cmatrix](https://github.com/abishekvashok/cmatrix) with the experimental "tron-disrupted" theme_

![Horizon screenshot](media/screenshot_horizon.png)

_Editing source code with `nvim` on the custom [`horizon-full`](https://github.com/GitSquared/horizon-edex-theme) theme_

## Q&A

#### How do I get it?
Clone the repo and build from source — see [Getting Started](#getting-started) below. Pre-built release binaries will follow as the project matures.

#### I have a problem!
Search through the [Issues](https://github.com/j3lly-Fish/hackterm/issues) to see if yours has already been reported. If not, open a new one.

#### Can you disable the keyboard/the filesystem display?
You can't disable them (yet) but you can hide them. See the `tron-notype` theme.

#### Why is the file browser saying "Tracking Failed"? (Windows only)
On Linux and macOS, hackterm tracks the terminal's current working directory to display folder contents on-screen. This is technically impossible on Windows right now, so the file browser falls back to a detached mode — you can still browse files and click to input paths into the terminal.

#### Can this run on a Raspberry Pi / ARM device?
See the upstream discussion in [eDEX-UI issue #313](https://github.com/GitSquared/edex-ui/issues/313#issuecomment-443465345) and [#818](https://github.com/GitSquared/edex-ui/issues/818) for ARM build guidance.

#### Is this actively maintained?
Yes — that's the whole point. The original eDEX-UI was archived in October 2021. hackterm exists to continue development. New features are being added and pull requests are welcome.

#### How did the original get made?
See [eDEX-UI issue #272](https://github.com/GitSquared/edex-ui/issues/272) for the original author's write-up.

## Getting Started

**IMPORTANT:** the following instructions run hackterm from source. Release binaries will be published separately once the project stabilizes.

#### Linux / macOS (requires Xcode command line tools on macOS):

```sh
git clone https://github.com/j3lly-Fish/hackterm.git
cd hackterm
npm run install-linux
npm run start
```

#### Windows (run as administrator):

```sh
git clone https://github.com/j3lly-Fish/hackterm.git
cd hackterm
npm run install-windows
npm run start
```

#### Building a distributable

Note: due to native modules, you can only build for the OS you are currently running.

```sh
npm install          # NOT install-linux/windows
npm run build-linux  # or build-darwin / build-windows
```

Output goes to the `dist/` folder.

#### Keyboard shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+Shift+S` | Open settings |
| `Ctrl+Shift+\` | Toggle split-pane mode |
| `Ctrl+Shift+E` | Open SSH connection dialog |
| `Ctrl+Shift+K` | View / edit keyboard shortcuts |
| `Ctrl+Tab` | Next terminal tab |
| `Ctrl+Shift+Tab` | Previous terminal tab |
| `Ctrl+Shift+C` | Copy |
| `Ctrl+Shift+V` | Paste |
| `Ctrl+Shift+F` | Fuzzy file search |

## Credits

hackterm is a fork of **[eDEX-UI](https://github.com/GitSquared/edex-ui)**, originally created by [Squared (Gabriel Saillard)](https://github.com/GitSquared). The vast majority of the codebase, the visual design, the themes, and the core architecture are his work. This fork would not exist without it.

[PixelyIon](https://github.com/PixelyIon) helped with the original Windows compatibility work.

[IceWolf](https://soundcloud.com/iamicewolf) composed the sound effects on v2.1.x and above.

The project builds on the shoulders of several excellent open-source libraries: [xterm.js](https://github.com/xtermjs/xterm.js), [systeminformation](https://github.com/sebhildebrandt/systeminformation), [SmoothieCharts](https://github.com/joewalnes/smoothie), and [Rob "Arscan" Scanlon](https://github.com/arscan)'s [ENCOM Globe](https://github.com/arscan/encom-globe).

## Licensing

Licensed under the [GPLv3.0](https://github.com/j3lly-Fish/hackterm/blob/master/LICENSE).
