# litmus

Terminal color theme previewer — Rust workspace with three crates:
- `crates/litmus-model` — shared data model
- `crates/litmus-cli` — TUI binary (ratatui + crossterm)
- `crates/litmus-web` — web frontend (Dioxus, targets wasm32)

## Rust Workflow

bacon is running in the background and continuously writes compiler
diagnostics to `.bacon-locations` in the project root.

Before attempting to fix compiler errors, read `.bacon-locations` to see
current errors and warnings with their exact file/line/column locations.
Prefer reading this file over running `cargo check` yourself — it's
already up to date and costs no compile time.

After making changes, wait a moment for bacon to recompile, then re-read
`.bacon-locations` to verify the fix.

If `.bacon-locations` is absent or clearly stale (e.g. the file doesn't
exist after the first save), warn the user that bacon does not appear to
be running and ask them to start it in a Zellij pane with `mise run bacon`.
