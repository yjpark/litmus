---
# litmus-ep4g
title: Uniform scene sizing with CSS aspect-ratio
status: todo
type: feature
priority: normal
created_at: 2026-03-25T15:05:26Z
updated_at: 2026-03-25T15:38:38Z
blocked_by:
    - litmus-dvjb
---

Simulated views expand to content while screenshots are fixed size, causing inconsistent heights within splits and across fixtures. Use CSS aspect-ratio on scene panels matching the screenshot capture ratio. ## Design: CSS aspect-ratio on scene panels

- Determine the screenshot aspect ratio from the capture dimensions (consistent across all screenshots since they use the same terminal size).
- Apply aspect-ratio on .scene-split-panel containers matching the screenshot ratio.
- Both simulated and screenshot panels scale responsively while maintaining the same proportions.
- Simulated content that overflows gets overflow: hidden with a fade-mask at the bottom (reuse the existing gradient mask pattern from .scene-preview).
- On the compare page, apply the same aspect-ratio constraint to grid items.

**Files:** style.css (aspect-ratio on .scene-split-panel, overflow + mask), theme_detail.rs (potentially pass aspect-ratio as inline style from manifest dimensions), compare.rs (same treatment for grid items)

## Tasks
- [ ] Determine screenshot aspect ratio from manifest capture dimensions
- [ ] Apply aspect-ratio on .scene-split-panel containers
- [ ] Add overflow: hidden + fade-mask for simulated content overflow
- [ ] Apply same aspect-ratio constraint to compare page grid items
- [ ] Test responsive behavior (panels should scale proportionally)
- [ ] Verify both detail page and compare page look uniform
