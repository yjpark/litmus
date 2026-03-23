# Litmus Next Stage — Provider/Consumer/Silo MVP Plan

## Context

Litmus currently has a solid foundation: 60 curated themes, 8 realistic scenes, a web app with filtering/comparison/CVD simulation/contrast checking, and config export. But the scene system is still "flat" — every scene is a standalone mock-up with hardcoded semantic color mappings.

The next stage introduces the **provider/consumer/silo** model described in `concepts.md`, making litmus the definitive tool for understanding how themes flow through real terminal ecosystems.

## Vision

A user should be able to:

1. **Pick a provider** (e.g., kitty, wezterm, alacritty) and see how a theme looks across all the consumer apps that run inside it
2. **Pick a consumer** (e.g., `git diff`, `delta`, `bat`) and see how it looks across different providers and themes
3. **See silos** (e.g., neovim with its own theme) rendered independently, with clear visual separation from the provider ecosystem
4. **Understand theme propagation** — which apps inherit colors from the provider, which define their own, and which can do both

## High-Level Architecture Changes

### 1. Provider Model

A provider defines the terminal color palette. The current `Theme` struct already models this well (16 ANSI + fg/bg/cursor/selection). What's missing:

- **Provider metadata**: which terminal emulator, what config format it uses, what consumers typically run inside it
- **Provider-specific export**: extend current export to cover wezterm, alacritty, foot, ghostty, Windows Terminal, etc.
- **Provider-specific scene context**: a scene running "inside" a provider may include provider chrome (tab bar, title bar, etc.)

### 2. Consumer Model

A consumer app uses the provider's ANSI palette. Consumers don't define themes — they reference ANSI colors by index. What's needed:

- **Consumer registry**: metadata about each consumer app (name, description, what ANSI colors it typically uses, how to configure it)
- **Consumer-aware scenes**: scenes tagged with which consumer app they represent. Current scenes already do this informally (the "Git Diff" scene is effectively the `git diff` consumer), but this should be explicit
- **Consumer configuration preview**: some consumers have their own color overrides (e.g., `delta` can remap ANSI colors, `LS_COLORS` defines per-filetype colors). The system should show how these interact with the provider's palette

### 3. Silo Model

A silo app defines its own theme independently. What's needed:

- **Silo theme definitions**: themes specific to silo apps (e.g., neovim colorschemes are NOT terminal themes — they're editor themes with their own palette)
- **Silo scene rendering**: a silo's scene uses the silo's own theme, not the provider's ANSI palette
- **Dual-mode support**: some apps (like `jjui`, `lazygit`) can run in either consumer mode (using terminal ANSI colors) or silo mode (using their own built-in theme). The UI should show both modes

### 4. Ecosystem View

The key new UI concept — showing an entire provider ecosystem together:

- Provider chrome (optional) at the top
- Multiple consumer scenes rendered with the same theme
- Silo apps rendered with their own theme, visually distinguished
- "What changes when I switch themes" — the ecosystem view should make it obvious which apps are affected by a theme change and which aren't

## Milestones

### M-Next-1: Provider & Consumer Data Model

**Goal:** Formalize the provider/consumer relationship in the data model.

**Tasks:**
- Define `Provider` struct (name, description, config format, supported export formats)
- Define `Consumer` struct (name, description, typical ANSI usage patterns)
- Define `Ecosystem` struct (provider + list of consumers)
- Tag existing scenes with their consumer app identity
- Create a registry of known providers (kitty, wezterm, alacritty, foot, ghostty, Windows Terminal, iTerm2)
- Create a registry of known consumers (git diff, delta, bat, fd, eza/ls, tig, htop, etc.)
- Unit tests for the data model

**Dependencies:** None (builds on existing litmus-model)

### M-Next-2: Ecosystem View UI

**Goal:** A new route/view that shows a complete provider ecosystem.

**Tasks:**
- New route: `/ecosystem/:provider` (e.g., `/ecosystem/kitty`)
- Ecosystem page layout: provider header + stacked consumer scenes
- Theme selector within the ecosystem view
- Show all consumer scenes rendered with the selected theme
- Navigation between ecosystems
- Link from theme detail page to "see this theme in ecosystems"

**Dependencies:** M-Next-1

### M-Next-3: Extended Export System

**Goal:** Export theme configs for all major providers, not just kitty.

**Tasks:**
- Alacritty TOML export
- Wezterm Lua export
- Foot INI export
- Ghostty config export
- Windows Terminal JSON export
- iTerm2 XML plist export
- Export page redesign: pick your provider, get the config
- "Copy to clipboard" for each format

**Dependencies:** M-Next-1 (needs Provider metadata)

### M-Next-4: Consumer Configuration Preview

**Goal:** Show how consumer-specific configs interact with themes.

**Tasks:**
- `LS_COLORS` / `EZA_COLORS` configuration model
- `delta` theme configuration model
- `bat` theme configuration model
- Preview: "here's what `ls` looks like with this LS_COLORS config under Theme X"
- Interactive: let users tweak consumer configs and see the result
- Document common consumer configurations

**Dependencies:** M-Next-1, M-Next-2

### M-Next-5: Silo App Support

**Goal:** Support apps that define their own themes independently of the terminal.

**Tasks:**
- Define `SiloTheme` type — distinct from terminal `Theme` (may have different color slots)
- Neovim colorscheme model (highlight groups, not just ANSI)
- Scene rendering with silo themes
- Dual-mode toggle: show `lazygit` in consumer mode vs its built-in theme
- Visual distinction in ecosystem view: "this app uses its own theme"
- Initial silo themes: a few popular neovim colorschemes

**Dependencies:** M-Next-2

### M-Next-6: Theme Relationship Visualization

**Goal:** Help users understand how themes flow through their setup.

**Tasks:**
- Diagram/visualization: provider → consumers (with arrows showing color inheritance)
- "What changes" diff view: switch themes and see highlighted differences
- Compatibility matrix: which themes work well in which ecosystems
- Theme conflict detection: when a silo theme clashes with the provider theme

**Dependencies:** M-Next-2, M-Next-5

## Technical Considerations

### Data Model Evolution

The current `Theme` struct maps cleanly to a "provider theme" — it IS the provider's palette. The key decision is how to model:

1. **Consumer color usage** — consumers don't have "themes", they have ANSI color references. The scene system already handles this via `ThemeColor::Ansi(u8)`. What's new is metadata about which consumer uses which colors.

2. **Silo themes** — these are structurally different from terminal themes. A neovim colorscheme has highlight groups (`Normal`, `Comment`, `String`, etc.) that map to arbitrary RGB colors, not ANSI indices. This needs a separate type.

3. **Ecosystem composition** — an ecosystem is a provider + consumers + optional silos. The data model should make it easy to add new consumers without changing the core types.

### Incremental Approach

Each milestone is independently shippable:
- M-Next-1 is a data model refactor with no UI changes
- M-Next-2 adds a new view without changing existing views
- M-Next-3 is purely additive (new export formats)
- M-Next-4 and M-Next-5 add depth to the ecosystem view
- M-Next-6 is the "delight" layer

### What NOT to Build

- **Live terminal capture**: running actual terminal commands in the browser is out of scope. All rendering remains simulated via the scene system.
- **Theme editor**: creating or modifying themes is not in scope. Litmus is a previewer, not an editor.
- **Plugin system**: extensibility via plugins is post-MVP. New providers/consumers are added to the codebase directly.
- **User accounts / preferences**: no server-side state. Everything runs client-side.

## Open Questions for Discussion

1. **Ecosystem naming**: Should ecosystems be named after providers ("kitty ecosystem") or be more generic ("terminal ecosystem")? Most consumers work across all terminal emulators, so the ecosystem is really "any terminal + these consumers."

2. **Consumer scene granularity**: Should each consumer get exactly one scene, or can a consumer have multiple scenes (e.g., `git diff` with conflicts, `git diff` with renames, `git log --graph`)?

3. **Silo theme sourcing**: Where do silo themes (e.g., neovim colorschemes) come from? The format is completely different from terminal themes. Do we curate a separate set, or try to auto-convert?

4. **Provider chrome**: Is rendering the provider's own UI (tab bar, scrollbar, etc.) worth the effort? It adds visual context but significantly increases scene complexity.

5. **Interactive consumer config**: How deep should the consumer configuration preview go? Full `LS_COLORS` editing is a substantial feature. Maybe start with presets ("default", "vivid", "minimal") rather than full interactivity.

6. **Priority ordering**: M-Next-3 (export) is high user value but independent of the ecosystem model. Should it be prioritized before M-Next-2?

## Summary

| Milestone | Effort | User Value | Risk |
|-----------|--------|------------|------|
| M-Next-1: Data Model | Medium | Low (foundation) | Low |
| M-Next-2: Ecosystem View | Medium | High | Medium |
| M-Next-3: Extended Export | Medium | High | Low |
| M-Next-4: Consumer Config | High | Medium | Medium |
| M-Next-5: Silo Support | High | Medium | High (new domain) |
| M-Next-6: Visualization | Medium | Medium | Low |

The recommended order is M-Next-1 → M-Next-3 → M-Next-2 → M-Next-4 → M-Next-5 → M-Next-6, prioritizing the export system (high standalone value, low risk) right after the data model.
