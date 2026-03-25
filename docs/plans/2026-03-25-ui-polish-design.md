# UI Polish: Provider Selection, Theme Availability & Scene Sizing

**Date:** 2026-03-25
**Status:** Approved

---

## Issue 1: Remove provider dropdown from detail page

**Problem:** The detail page has its own local `selected_provider` signal and dropdown, separate from the global `ActiveProvider` in the sidebar. This is leftover from before the UI rework.

**Design:** Remove the local signal and dropdown from `theme_detail.rs`. Read the provider from the global `ActiveProvider` context signal instead. Single source of truth.

**Files:** `theme_detail.rs`, `style.css` (remove `.detail-provider-select` / `.detail-provider-label`)

---

## Issue 2: Show all themes with availability feedback

**Problem:** `themes_for_provider()` returns only themes with colors for the active provider. Switching providers causes the theme list to refresh dramatically, which is jarring.

**Design: Dimmed cards with unavailable badge (Approach A)**

- Replace `themes_for_provider()` with a function that returns **all themes**, each annotated with an `available: bool` flag for the current provider.
- **Available themes:** Render normally, clickable, navigate to detail page.
- **Unavailable themes:** Reduced opacity (0.4–0.5), small "unavailable" badge or icon overlay, `cursor: not-allowed`, click suppressed (no navigation).
- Alphabetical sort preserved — unavailable themes stay in their natural position, not pushed to the bottom.
- Filter counts (variant toggles, total) reflect **only available themes** for the active provider.
- Search/variant/readability filters still apply to all themes (available and unavailable).

**Files:** `themes.rs` (new `all_themes_with_availability()`), `theme_list.rs` (card rendering + click gating), `state.rs` (filter counts), `style.css` (dimmed card styles)

---

## Issue 3: Uniform scene section sizing via aspect-ratio

**Problem:** Simulated views expand to fit content while screenshots have fixed capture dimensions. This creates inconsistent heights both within a split (simulated vs screenshot) and across fixture sections on the detail page.

**Design: CSS `aspect-ratio` on scene panels (Approach B)**

- Determine the screenshot aspect ratio from the capture dimensions (consistent across all screenshots since they use the same terminal size).
- Apply `aspect-ratio` on `.scene-split-panel` containers matching the screenshot ratio.
- Both simulated and screenshot panels scale responsively while maintaining the same proportions.
- Simulated content that overflows gets `overflow: hidden` with a fade-mask at the bottom (reuse the existing gradient mask pattern from `.scene-preview`).
- On the compare page, apply the same aspect-ratio constraint to grid items.

**Files:** `style.css` (aspect-ratio on `.scene-split-panel`, overflow + mask), `theme_detail.rs` (potentially pass aspect-ratio as inline style from manifest dimensions), `compare.rs` (same treatment for grid items)
