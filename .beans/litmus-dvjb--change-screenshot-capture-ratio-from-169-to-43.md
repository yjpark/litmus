---
# litmus-dvjb
title: Change screenshot capture ratio from 16:9 to 4:3
status: in-progress
type: task
priority: normal
created_at: 2026-03-25T15:38:34Z
updated_at: 2026-03-25T15:50:35Z
---

Current screenshots are 1280x720 (16:9), which is too wide for terminal content — leaves dead space on the right. Switch to 4:3 (e.g. 960x720 or 1024x768) for a more natural terminal aspect ratio.

## Tasks
- [x] Decide exact capture dimensions: 80 cols × 32 rows → ~1280×960 (4:3 at 2x)
- [x] Update capture config/code with new terminal window size
- [ ] Re-capture all screenshots for both providers (kitty, wezterm)
- [ ] Rebuild manifest
- [ ] Verify screenshots look good at new ratio

## Blocked

Code changes done (TermGeometry default changed from 24 to 32 rows). Re-capture requires Wayland + GPU environment:
```bash
mise run capture-kitty
mise run capture-wezterm
mise run capture-manifest
```
Then verify the new screenshots have ~4:3 ratio and look good.
