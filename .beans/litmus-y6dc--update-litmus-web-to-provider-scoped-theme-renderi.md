---
# litmus-y6dc
title: Update litmus-web to provider-scoped theme rendering
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:23:07Z
updated_at: 2026-03-24T14:12:39Z
parent: litmus-knrz
blocked_by:
    - litmus-jmna
    - litmus-o4w9
---

Migrate litmus-web from the old Theme struct to ThemeDefinition + ProviderColors.

## Global Provider Selector

Provider becomes **app-level state**, not a per-page control. Switching provider changes simulated colors, screenshots, contrast validation, and which themes are visible.

**UI:** Segmented control (buttons, not dropdown) at the top of sidebar, above all filters. E.g. `[kitty] [wezterm]`. Styled as a prominent first-class mode selector.

**State persistence:**
- URL parameter takes precedence (e.g. `/themes?provider=kitty`)
- Falls back to localStorage
- Falls back to first available provider
- Shareable/bookmarkable links

**Filtering:** Selecting a provider filters the theme list to only themes with ProviderColors for that provider. Themes without colors for the selected provider are hidden everywhere (list, detail, compare).

**Navigation edge case:** If user is on a detail page and switches to a provider that doesn't have that theme, redirect to theme list.

## Data Loading

- Update load_embedded_themes() to return Vec<ThemeDefinition> + per-provider color map
- Theme only listed if it has ProviderColors for the active provider

## Rendering

- Simulated scenes render using ProviderColors for the active provider
- Screenshots use active provider
- Contrast validation scoped to active provider's colors
- Theme list cards show previews using active provider's colors
- Compare page uses active provider for all columns

## State Management

- New global signal: `ActiveProvider(String)` (like existing CvdSimulation, VariantFilter)
- Remove per-page provider dropdowns (detail page, compare page)
- All components read from global ActiveProvider

Depends on: new model types, converted themes
