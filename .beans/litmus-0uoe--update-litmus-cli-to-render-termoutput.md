---
# litmus-0uoe
title: Update litmus-cli to render TermOutput
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:47:32Z
updated_at: 2026-03-25T00:07:43Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-9eg8
---

Migrate litmus-cli mockup views from Scene to TermOutput:

- Load TermOutput data (bundled or from fixtures directory)
- Render TermSpan using crossterm colors:
  - TermColor::Default → crossterm reset
  - TermColor::Ansi(n) → crossterm AnsiValue
  - TermColor::Indexed(n) → crossterm AnsiValue
  - TermColor::Rgb → crossterm Rgb
- Minimal changes — CLI is simpler than web

Depends on: TermOutput types, fixture pipeline generating output files

## Plan

1. Add `resolve_with_theme()` to `TermColor` in litmus-model (reuses existing `indexed_color()`)
2. Add `serde_json` dep to litmus-cli
3. Write tests: `resolve_with_theme` unit tests, TermOutput-to-ratatui-Lines conversion test
4. Embed 3 fixture output.json files (git-diff, ls-color, shell-prompt) via `include_str!()`
5. Rewrite MockupsWidget to parse embedded fixtures, map TermSpan → ratatui Span using theme colors
6. Add `term_color_to_ratatui()` helper in util.rs
