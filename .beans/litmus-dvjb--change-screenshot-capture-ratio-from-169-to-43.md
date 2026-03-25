---
# litmus-dvjb
title: Change screenshot capture ratio from 16:9 to 4:3
status: todo
type: task
created_at: 2026-03-25T15:38:34Z
updated_at: 2026-03-25T15:38:34Z
---

Current screenshots are 1280x720 (16:9), which is too wide for terminal content — leaves dead space on the right. Switch to 4:3 (e.g. 960x720 or 1024x768) for a more natural terminal aspect ratio.

## Tasks
- [ ] Decide exact capture dimensions (960x720 or 1024x768)
- [ ] Update capture config/code with new terminal window size
- [ ] Re-capture all screenshots for both providers (kitty, wezterm)
- [ ] Rebuild manifest
- [ ] Verify screenshots look good at new ratio
