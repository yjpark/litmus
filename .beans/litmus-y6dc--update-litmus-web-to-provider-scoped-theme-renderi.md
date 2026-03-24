---
# litmus-y6dc
title: Update litmus-web to provider-scoped theme rendering
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T13:23:07Z
updated_at: 2026-03-24T15:52:26Z
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

## Plan

### Phase 1: Update themes.rs data loading
- Replace single THEME_DATA array with two arrays: DEFINITION_DATA and PROVIDER_COLORS_DATA
- Parse using parse_theme_definition() and parse_provider_colors()
- Add load_embedded_theme_data() → (Vec<ThemeDefinition>, HashMap<Key, ProviderColors>)
- Add available_providers() → sorted list of provider slugs
- Add themes_for_provider(provider) → Vec<Theme>

### Phase 2: Add ActiveProvider signal to state.rs
- New signal: ActiveProvider(String)
- URL param: ?provider=kitty
- localStorage fallback
- Default: first available provider

### Phase 3: Update components
- Shell: theme chrome uses active provider
- Sidebar: add provider selector (segmented control)
- ThemeList: filter by active provider
- ThemeDetail: use active provider for rendering + screenshots
- Compare: use active provider
- SceneAcross: use active provider

### Phase 4: Navigation edge cases
- Provider switch on detail page → redirect if theme unavailable

### Todo
- [ ] Update themes.rs with provider-based loading
- [ ] Add ActiveProvider signal
- [ ] Add provider selector UI
- [ ] Update all pages to filter by provider
- [ ] Handle navigation edge cases
- [ ] Compiles for wasm32, zero warnings
