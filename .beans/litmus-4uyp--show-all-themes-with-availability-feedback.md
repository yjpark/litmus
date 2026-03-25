---
# litmus-4uyp
title: Show all themes with availability feedback
status: todo
type: feature
priority: normal
created_at: 2026-03-25T15:05:23Z
updated_at: 2026-03-25T15:06:54Z
---

Theme list only shows themes for the active provider — switching providers refreshes the list jarringly. Show all themes instead with dimmed cards + badge for unavailable ones. ## Design: Dimmed cards with unavailable badge

- Replace themes_for_provider() with a function that returns **all themes**, each annotated with an available: bool flag for the current provider.
- **Available themes:** Render normally, clickable, navigate to detail page.
- **Unavailable themes:** Reduced opacity (0.4–0.5), small "unavailable" badge or icon overlay, cursor: not-allowed, click suppressed (no navigation).
- Alphabetical sort preserved — unavailable themes stay in their natural position, not pushed to the bottom.
- Filter counts (variant toggles, total) reflect **only available themes** for the active provider.
- Search/variant/readability filters still apply to all themes (available and unavailable).

**Files:** themes.rs (new all_themes_with_availability()), theme_list.rs (card rendering + click gating), state.rs (filter counts), style.css (dimmed card styles)

## Tasks
- [ ] Add all_themes_with_availability() returning (Theme, bool) tuples
- [ ] Update theme_list.rs to render unavailable cards with reduced opacity
- [ ] Add 'unavailable' badge/icon overlay on dimmed cards
- [ ] Suppress click/navigation for unavailable themes (cursor: not-allowed)
- [ ] Keep alphabetical sort with unavailable themes in natural position
- [ ] Filter counts reflect only available themes
- [ ] Add CSS for dimmed card state (.theme-card--unavailable or similar)
- [ ] Test that search/variant/readability filters apply to all themes
