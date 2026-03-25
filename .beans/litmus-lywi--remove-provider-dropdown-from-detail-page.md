---
# litmus-lywi
title: Remove provider dropdown from detail page
status: todo
type: bug
priority: normal
created_at: 2026-03-25T15:05:20Z
updated_at: 2026-03-25T15:06:51Z
---

The detail page has its own local selected_provider signal and dropdown, leftover from before the UI rework. Remove it and read from global ActiveProvider instead. ## Design

Remove the local signal and dropdown from theme_detail.rs. Read the provider from the global ActiveProvider context signal instead. Single source of truth.

**Files:** theme_detail.rs, style.css (remove .detail-provider-select / .detail-provider-label)

## Tasks
- [ ] Remove local selected_provider signal from theme_detail.rs
- [ ] Remove provider dropdown UI from theme_detail.rs
- [ ] Use global ActiveProvider context signal for screenshot lookups
- [ ] Remove .detail-provider-select / .detail-provider-label CSS
- [ ] Verify detail page works with sidebar provider switching
