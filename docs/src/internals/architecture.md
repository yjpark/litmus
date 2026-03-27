# Architecture

Litmus is a Rust workspace with four crates.

## Crates

| Crate | Role | Target |
|-------|------|--------|
| `litmus-model` | Shared types and logic | any |
| `litmus-web` | Web frontend | wasm32 |
| `litmus-cli` | TUI prototype | native |
| `litmus-capture` | Screenshot capture pipeline | native (Wayland) |

### litmus-model

The foundation crate — no UI dependencies, compiles for any target. Contains:

- **Theme types** — `ThemeDefinition`, `ProviderColors`, `Color`, `AnsiColors`
- **Screenshot model** — `Provider`, `Fixture`, `ScreenshotKey`, `ScreenshotManifest`
- **TermOutput** — `TermColor`, `TermSpan`, and color resolution logic
- **Contrast** — WCAG 2.1 ratio calculation, APCA scoring, per-fixture validation
- **CVD** — color vision deficiency simulation matrices
- **Parsers** — TOML theme format, kitty.conf, base16 YAML, wezterm
- **Export** — kitty.conf, TOML, Nix output formatters

### litmus-web

Dioxus WASM application — the primary interface. Depends on `litmus-model`. Themes and fixture data are embedded at compile time via `include_str!`.

### litmus-cli

The original ratatui + crossterm TUI prototype from milestone 0. Still maintained but secondary to the web app.

### litmus-capture

Native binary for headless screenshot capture. Requires a Wayland compositor with GPU access. Contains the ANSI parser, capture orchestration, color extraction, and manifest builder. See [Capture Pipeline](./capture.md).

## Data Flow

```
Theme TOML files
  ├─ theme definition (name, variant, providers map)
  └─ provider color files (.kitty.toml, .wezterm.toml)
        ↓
  litmus-web embeds at compile time
        ↓
  ThemeDefinition + ProviderColors (in-memory)
        ↓
  TermColor::resolve() maps semantic refs → RGB
        ↓
  CSS inline styles (simulated rendering)

Fixture scripts (setup.sh + command.sh)
  ├─ litmus-capture runs them in headless terminals → WebP screenshots → R2
  └─ litmus-capture parses ANSI output → output.json → embedded in litmus-web
        ↓
  TermOutput + ProviderColors → simulated rendering
  Screenshot manifest → lazy-loaded images
```

## Routing

All routes are provider-scoped:

| Route | Page | Purpose |
|-------|------|---------|
| `/:provider/` | ThemeList | Home — filterable theme grid |
| `/:provider/theme/:slug` | ThemeDetail | Single theme, all fixtures |
| `/:provider/scene/:scene_id` | SceneAcrossThemes | One fixture across all themes |
| `/:provider/compare/:slugs` | CompareThemes | Two themes side by side |

The provider prefix ensures that switching between kitty and wezterm changes the URL, making provider-specific views linkable and bookmarkable.

## Deployment

The web app deploys to **Cloudflare Pages** via GitHub Actions:

1. Build with `dx build --release` (Dioxus CLI)
2. Inject critical CSS to prevent flash of unstyled content during WASM load
3. Deploy with Wrangler to `litmus.edger.dev`

Screenshots deploy separately to **Cloudflare R2** at `screenshots.litmus.edger.dev`. The manifest and images are synced via `rclone` with appropriate cache headers.

## State Management

The web app uses Dioxus signals for reactive state:

- `ActiveProvider` — selected terminal emulator (kitty/wezterm)
- `Favorites` — starred themes (up to 20)
- `VisitHistory` — last 5 viewed themes
- `LastComparedSlug` — previous compare partner for the "vs." button
- `FilterState` — search query, variant filter, readability threshold
- `CvdSimulation` — active CVD mode (none/protanopia/deuteranopia/tritanopia)
- `ManifestState` — cached screenshot manifest from CDN
- `AppThemeSlug` — the theme used for the app's own UI chrome
