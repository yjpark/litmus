---
# litmus-c55s
title: Build color swatch and color showcase fixtures
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T14:01:30Z
updated_at: 2026-03-24T15:42:23Z
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

## Plan

### color-swatch fixture
A shell script that prints a reference palette:
- 16 ANSI colors as labeled foreground text and background blocks
- 256-color palette grid (6×6×6 cube sampling + grayscale)
- A few truecolor gradient samples
- setup.sh is empty (no state needed), command.sh does all output

### color-showcase fixture
A simulated CI/deploy dashboard that naturally uses all 16 ANSI colors:
- Build status lines (green pass, red fail, yellow warning)
- Deploy pipeline stages
- Service health checks
- Deterministic, self-contained printf/echo output

### Todo
- [ ] Create color-swatch fixture (setup.sh + command.sh)
- [ ] Create color-showcase fixture (setup.sh + command.sh)
- [ ] Test both fixtures locally
- [ ] Update README.md fixture inventory
- [ ] Review
