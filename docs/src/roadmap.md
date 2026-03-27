# Roadmap

## What's Shipped

Litmus has gone through five releases, evolving from a TUI prototype to a full-featured web app:

**v0.1** — Initial release. Rust workspace with three crates, 19 curated themes, scene-based rendering, Dioxus WASM web app with theme browsing and detail views.

**v0.2** — Major expansion. Image-backed screenshot system with headless capture for kitty and wezterm. APCA readability scoring, CVD simulation, config export (kitty.conf/TOML/Nix), sidebar layout, app theming, theme count expanded to 60.

**v0.3** — Architecture shift. Migrated from handcrafted scenes to `TermOutput` model based on real ANSI capture. Provider-scoped routing (`/:provider/`), R2 screenshot deployment, interactive contrast issue navigation, fixture anchor deep-links.

**v0.4** — Compare redesign. Strict 2-theme side-by-side comparison with inline pickers. Favorites (star toggle, 20 cap), visit history, per-fixture contrast issue dots in sidebar minimap.

**v0.5** — Polish. Compare history tracking, sidebar label improvements.

See [CHANGELOG.md](https://github.com/edger-dev/litmus/blob/main/CHANGELOG.md) for full details.

## Next Major Version

The next major version extends the provider/consumer model with silo support and broadens provider coverage.

### Silo Support

Apps like neovim and helix define their own color palettes independent of the terminal. The silo model will:

- Introduce a separate theme type for app-specific palettes (e.g. neovim highlight groups)
- Render silo fixtures using the silo's own palette
- Support dual-mode apps (lazygit, jjui) that can operate as consumer or silo
- Visually distinguish provider-affected output from silo output in an ecosystem view

See [Silos (Roadmap)](./model/silos.md) for design details.

### Ecosystem View

A new page showing a complete provider ecosystem: the terminal emulator plus all consumer apps rendered together. This makes it visible which apps change when you switch themes and which don't.

### Extended Export

Config generation for additional providers:

- Alacritty (TOML)
- Wezterm (Lua)
- Foot (INI)
- Ghostty (config)
- Windows Terminal (JSON)
- iTerm2 (XML plist)

### Consumer Configuration Preview

Show how consumer-specific settings interact with themes — e.g. how `LS_COLORS` or `delta` config affects the appearance under different themes. Starting with presets rather than full interactivity.

## Not in Scope

These are explicitly out of scope for litmus:

- **Live terminal capture in browser** — all rendering is from pre-captured data, not live terminal sessions
- **Theme editor** — litmus is a previewer, not a theme creation tool
- **Plugin system** — new providers, consumers, and fixtures are added to the codebase directly
- **User accounts** — no server-side state; everything runs client-side
