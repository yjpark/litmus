# Litmus

> Litmus test your terminal themes — preview them across all your apps before you commit.

## The Problem

Switching terminal themes is a frustrating loop:

1. You find a theme that looks nice in your terminal's preview
2. You edit configs for kitty/wezterm, neovim, zellij, tig, delta...
3. You discover `git diff` is unreadable, or jjui's text blends into the background
4. You revert everything and try the next theme
5. Repeat

The core issue: **you can't see how a theme actually looks across your real workflow until after you've fully set it up.** Kitty's built-in theme preview shows ANSI swatches, but that doesn't tell you whether a complex `git diff` or a tig log view will be readable.

## What Litmus Does

A web app that lets you **preview any theme across realistic terminal scenes instantly** — before touching a single config file.

### Features

- **19 curated themes** from 8 families (Catppuccin, Tokyo Night, Gruvbox, Solarized, Rose Pine, Everforest, Dracula, Nord, Kanagawa)
- **5 realistic terminal scenes**: shell prompt, git diff, directory listing, cargo build output, git log with branch graph
- **WCAG contrast validation**: automatic readability checks for every color pair in every scene
- **Side-by-side comparison**: compare two themes across the same scenes
- **Browse by theme or by scene**: theme-first (pick a theme, see all scenes) or scene-first (pick a scene, see all themes)
- **Family grouping**: themes organized by family on the listing page
- **Light/dark detection**: automatic variant labeling with contrast ratios

## Getting Started

### Prerequisites

- Rust toolchain (edition 2024)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) for the web frontend

### Running the CLI

```sh
cargo run -p litmus-cli
```

Navigate themes with arrow keys, switch views with Tab, press `q` to quit.

### Running the Web App

```sh
dx serve --project crates/litmus-web
```

Opens a local web server with the theme browser.

## Project Structure

```
crates/
  litmus-model/    Shared data model: Theme, Scene, contrast validation
  litmus-cli/      TUI theme previewer (ratatui + crossterm)
  litmus-web/      Web theme browser (Dioxus, targets wasm32)
themes/            19 curated themes in TOML format
```

### Key Modules

- **`litmus_model::scene`** — Scene format: `ThemeColor` (semantic color refs), `StyledSpan`, `SceneLine`, `Scene`
- **`litmus_model::scenes`** — 5 built-in terminal scenes
- **`litmus_model::contrast`** — WCAG 2.1 contrast ratio calculation and scene validation
- **`litmus_web::scene_renderer`** — Dioxus components that render scenes as styled HTML

### Theme Format

Themes use a simple TOML format with 16 ANSI colors plus background, foreground, cursor, and selection colors. See `themes/` for examples. Parsers also support kitty.conf and base16 YAML.

## Architecture

### Semantic Color References

Scenes don't hardcode colors. Instead, each text span references a `ThemeColor` — `Foreground`, `Background`, `Ansi(0)` through `Ansi(15)`, `Cursor`, `SelectionBackground`, or `SelectionForeground`. At render time, these resolve against the current theme. This means scenes are defined once and render correctly for any theme.

### Provider/Consumer Model

Terminal apps relate to themes in fundamentally different ways:

- **Providers** (kitty, wezterm, alacritty) define the complete color palette
- **Consumers** (git diff, delta, ls, tig, bat) inherit colors from the provider via ANSI codes
- **Silos** (some apps with built-in themes) define their own isolated palette

Litmus scenes model the consumer perspective — showing how standard terminal output looks under any provider's palette.

## Future Directions

- Live terminal playground (xterm.js)
- Config generation (kitty.conf, home-manager modules)
- Editor ecosystem previews (neovim, helix)
- Theme forking and customization
- Color blindness simulation
- Community-submitted scenes

## Contributing

_This project is in active development._
