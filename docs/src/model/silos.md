# Silos (Roadmap)

> Silo support is planned for litmus's next major version. This page describes the design direction, not current functionality.

A **silo** is an app that defines its own color palette, independent of the terminal provider. Changing your terminal theme has no effect on a silo — it controls its own colors entirely.

## Examples

- **Neovim** with a colorscheme (e.g. tokyonight.nvim) — highlight groups map to arbitrary RGB, not ANSI indices
- **Helix** with a theme — same idea, editor-internal palette
- **lazygit** in built-in theme mode — uses its own color definitions instead of terminal ANSI

## Why Silos Need Special Treatment

Silo themes are structurally different from provider themes. A neovim colorscheme defines highlight groups (`Normal`, `Comment`, `String`, `Error`, etc.) with arbitrary RGB values. These don't map to the 16 ANSI palette — they exist in a completely separate color space.

This means:

- Silo themes can't be previewed using provider color data
- A silo's appearance is unaffected by provider theme changes
- The ecosystem view needs to visually distinguish "this changes with your theme" from "this stays the same"

## Planned Features

- **Silo theme definitions** — a separate type modeling app-specific palettes (e.g. neovim highlight groups)
- **Silo scene rendering** — fixtures that use the silo's own palette instead of the provider's
- **Dual-mode toggle** — for apps like lazygit that can operate as consumer or silo, show both modes side by side
- **Ecosystem view integration** — clear visual separation between provider-affected consumers and independent silos
