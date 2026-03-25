---
# litmus-lm76
title: Update litmus-web to render TermOutput instead of Scene
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:47:28Z
updated_at: 2026-03-25T00:15:52Z
parent: litmus-coma
blocked_by:
    - litmus-q9lp
    - litmus-9eg8
---

Migrate litmus-web from Scene to TermOutput rendering:

- Embed output.{provider}.json files via include_str!() or include_bytes!()
- New TermOutputView component replaces SceneView:
  - Renders TermSpan as <span> with inline CSS color
  - TermColor::Default → theme fg/bg from ProviderColors
  - TermColor::Ansi(n) → ProviderColors ANSI palette
  - TermColor::Indexed(n) → fixed 256-color → rgb CSS
  - TermColor::Rgb → literal rgb CSS
  - Bold/italic/dim/underline → CSS font-weight/style/opacity/decoration
- Provider selector switches which output.{provider}.json is rendered
- Update theme_detail.rs side-by-side to use TermOutputView on the left
- Update ScenePreview (theme list cards) to use TermOutput

Depends on: TermOutput types, fixture pipeline generating output files


## Plan

1. Create `crates/litmus-web/src/fixtures.rs` — embed fixture output.json files, parse with LazyLock
2. Create `crates/litmus-web/src/term_renderer.rs` — TermOutputView and TermOutputPreview components
3. Wire TermOutputView into theme_detail.rs (replace SceneView on left panel)
4. Wire TermOutputPreview into theme_list.rs (replace ScenePreview in cards)
5. Wire TermOutputView into scene_across.rs and compare.rs
6. Tests for color resolution and fixture loading
