# Dev Setup

## Prerequisites

- **Rust toolchain** — stable channel with the `wasm32-unknown-unknown` target, as defined in `rust-toolchain.toml`
- **mise** — task runner for all dev commands
- **Nix** (optional) — `flake.nix` provides a reproducible dev shell with all dependencies

For screenshot capture only (not needed for web development):
- Wayland compositor with GPU access
- `grim` for screenshots
- kitty and/or wezterm installed

## Quick Start

```bash
# Start the web app (Dioxus dev server, port 8883)
mise run dev

# Start with local screenshot serving (port 8884 for images, 8883 for app)
mise run dev-screenshots

# Build for production
mise run build-web
```

## Development Workflow

### Bacon Diagnostics

The recommended workflow uses **bacon** for continuous compilation feedback:

1. Start bacon in a terminal pane: `mise run bacon-claude-diagnostics`
2. Edit code
3. Bacon watches for changes and writes diagnostics to `.bacon-claude-diagnostics`
4. Read that file for errors with exact file/line/column locations

Each line uses a pipe-delimited format:

```
level|:|file|:|line_start|:|line_end|:|message|:|rendered
```

This is faster than running `cargo check` (bacon is already watching) and machine-parseable.

### Useful mise Tasks

| Task | Description |
|------|-------------|
| `dev` | Start Dioxus dev server (port 8883) |
| `dev-screenshots` | Local screenshot server + web app |
| `build-web` | Build WASM release |
| `build-cli` | Build CLI release |
| `check` | `cargo check` across workspace |
| `fmt` | Format with `cargo fmt` |
| `docs-serve` | Serve mdbook with live reload (port 8882) |
| `docs-build` | Build static docs |
| `capture-kitty` | Capture all kitty screenshots |
| `capture-wezterm` | Capture all wezterm screenshots |
| `screenshots-deploy` | Build manifest + sync to R2 |

## Work Tracking

Litmus uses **beans**, a file-based issue tracker. Beans are Markdown files with YAML frontmatter, managed via the `beans` CLI. Work is organized into milestones with parent/child and blocking relationships.

```bash
beans list --ready     # see what's available to work on
beans show <id>        # read a bean's full spec
beans create "Title" -t task -d "Description"
beans update <id> -s in-progress
```
