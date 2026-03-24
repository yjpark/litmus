---
# litmus-c55s
title: Build color swatch and color showcase fixtures
status: todo
type: task
created_at: 2026-03-24T14:01:30Z
updated_at: 2026-03-24T14:01:30Z
parent: litmus-49jz
---

Create two special-purpose fixtures:

**color-swatch/** — raw reference palette:
- Small Rust or shell program
- Prints all 16 ANSI colors (0-15) as labeled foreground text + background blocks
- 256-color palette grid (6x6x6 cube + 24 grayscale)
- A few truecolor gradients (red→green, dark→light, hue wheel)
- Must fit 80x24

**color-showcase/** — themed palette in context:
- Looks like a real scenario (e.g. status dashboard, CI pipeline summary)
- Naturally exercises all 16 ANSI colors including brights
- Uses background colors for status bars/highlights
- Deterministic output
