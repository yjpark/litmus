# The Model

Terminal apps don't all get their colors the same way. Understanding this is the key insight behind litmus — and the reason a simple color swatch preview doesn't cut it.

## Why Switching Themes Is Broken

When you change your terminal theme, some apps change with it and some don't. `git diff` picks up the new colors immediately. Your neovim colorscheme stays exactly the same. And `lazygit` might use its own built-in palette or fall back to terminal colors depending on how you configured it.

This is confusing because it's invisible. You change one config file and expect a uniform result, but the actual outcome depends on a hidden property of each app: **where it gets its colors from.**

Litmus makes this visible. It models terminal color relationships explicitly, so you can preview exactly what will change — and what won't — when you switch themes.

## Three Roles

Every terminal app falls into one of three roles:

- **[Providers](./providers.md)** — apps that define a complete color palette (terminal emulators like kitty, wezterm)
- **[Consumers](./consumers.md)** — apps that inherit colors from their provider (git diff, cargo, bat, ls)
- **[Silos](./silos.md)** — apps that bring their own independent palette (neovim with a colorscheme) *(roadmap)*

## Ecosystems

The provider/consumer relationship creates natural **ecosystems**. A terminal emulator (provider) plus all the CLI tools running inside it (consumers) form a group where colors flow in one direction: from provider to consumers.

When you preview a theme in litmus, you're seeing an entire ecosystem. The 13 fixtures represent different consumer apps — git diff, cargo build, bat, ripgrep, ls, and more — all rendered with the same provider palette. This is the core insight: **a useful theme preview shows the ecosystem, not just the palette.**

Switching between providers (kitty vs wezterm) in litmus shows you that the same theme can look subtly different depending on which terminal you use — because providers sometimes interpret theme definitions differently.

## Dual-Mode Apps

Some apps can operate as either a consumer or a silo. `lazygit` can use terminal ANSI colors (consumer mode) or its own built-in theme (silo mode). `jjui` behaves similarly.

Litmus will support showing both modes for dual-mode apps in a future version, making it clear what changes when you switch themes and what stays the same.
