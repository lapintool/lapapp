# Lapapp

[English](./README.md) | [简体中文](./README.zh.md)

A desktop application starter built on **Tauri 2 + Vue 3 + TypeScript + [lapstyle](../../Web/lapstyle)** — a clean starting point for new projects.

- Frontend styles and components come from lapstyle (the `Ls*` component kit + CSS design tokens)
- The borderless window and window-state persistence follow the approach proven in Lapeditor

## Features

- **Borderless window**: on Windows the system decorations are removed and a custom titlebar is rendered — fully draggable, double-click to maximize; other platforms keep native decorations
- **Window state memory**: position, size and maximized state are saved automatically (debounced 1s after move/resize + a final save on close) and restored before the window becomes visible — no jumping
- **Zoom**: Ctrl + `+` / `-` / `0` and Ctrl + mouse wheel (0.5–2.0), remembered in `settings.json`
- **Themes**: all 7 lapstyle themes (dark / light / mint / sky / pink / brown / amber), one click on the round button in the top-right corner
- **Multi-language**: Simplified Chinese / English built in, switched instantly from the top-right corner
- **UI example pages**: Widgets (lapstyle controls showcase) and Settings (theme / language / about)

Theme and language are persisted to `config/settings.json`; Rust injects the theme attribute before any page JavaScript runs to avoid a flash on startup.

## Requirements

- Node.js 20+ and [pnpm](https://pnpm.io)
- Rust (stable, with the MSVC toolchain on Windows)
- The lapstyle repository must be located at `../../Web/lapstyle` (relative to this project), see the `file:` dependency in `package.json`

## Getting started

```bash
pnpm install
pnpm dev        # dev mode (frontend runs on port 1430, avoiding Lapeditor's 1420)
pnpm build      # build installers
```

Other commands:

| Command | Description |
| --- | --- |
| `pnpm dev:web` | Frontend only (browser debugging, no Tauri shell) |
| `pnpm build:web` | Type check + frontend production build |
| `pnpm typecheck` | vue-tsc type check |
| `pnpm tauri icon <svg>` | Regenerate app icons for all platforms |

## Project structure

```
├── index.html                  # Entry HTML (inline pre-paint theme script, anti-flash)
├── src/
│   ├── main.ts                 # App entry: lapstyle CSS/plugin + i18n + window-state binding
│   ├── App.vue                 # Layout shell: titlebar + sidebar + content area
│   ├── styles.css              # App styles (layered on top of lapstyle tokens)
│   ├── settings.ts             # Theme/locale state and persistence (Rust commands)
│   ├── i18n.ts                 # vue-i18n setup and locale list
│   ├── windowState.ts          # Debounced save of window geometry on move/resize
│   ├── zoom.ts                 # Ctrl+±/0 and Ctrl+wheel zoom shortcuts
│   ├── components/
│   │   ├── TitleBar.vue        # Custom titlebar: round language/theme buttons + window controls
│   │   └── SideNav.vue         # Left sidebar menu
│   ├── views/                  # Pages (rendered in the content area per menu selection)
│   │   ├── WidgetsView.vue
│   │   └── SettingsView.vue
│   └── locales/                # zh.ts / en.ts messages
└── src-tauri/
    ├── tauri.conf.json         # Default window size, devUrl (1430), bundle config
    ├── capabilities/default.json
    └── src/
        ├── lib.rs              # Tauri setup: borderless, geometry restore, command registry
        ├── config.rs           # settings.json / window.json read-write and restore logic
        └── paths.rs            # Portable layout (config/ next to exe / repo root)
```

## Adding a page

1. Create a component in `src/views/`, e.g. `AboutView.vue`
2. Register it in the `pages` map of `src/App.vue`: `about: AboutView`
3. Add a `<ls-menu-item value="about">` entry in `src/components/SideNav.vue`
4. Add the messages to `src/locales/zh.ts` and `en.ts`

## Runtime configuration

Portable by design — everything lives in `config/` under the run directory (the repo root during development):

| File | Contents |
| --- | --- |
| `window.json` | `x / y / width / height / maximized` |
| `settings.json` | `theme / locale` |

## Implementation notes (ported from Lapeditor)

- Tauri's `set_size()` sets the *inner* size while `set_position()` sets the *outer* position; when maximized, the restore bounds are read via Win32 `GetWindowPlacement` and converted by the decoration delta — otherwise the window would grow by the DWM frame on every launch
- Minimized windows on Windows report coordinates like `-32000`; both save and restore validate against garbage values
- The window starts with `visible: false`; geometry is restored before `show()` to avoid visible repositioning
- The custom titlebar is used only on Windows (via the `uses_custom_titlebar` command); other platforms keep native decorations
