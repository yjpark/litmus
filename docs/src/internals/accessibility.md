# Accessibility

Litmus includes built-in tools for evaluating theme accessibility: contrast checking, readability scoring, and color vision deficiency simulation.

## WCAG Contrast Checking

`crates/litmus-model/src/contrast.rs` implements WCAG 2.1 contrast ratio calculation.

The process:

1. Convert sRGB to linear luminance using the standard gamma transfer function
2. Compute relative luminance for each color
3. Calculate the contrast ratio between foreground and background
4. Compare against thresholds: **AA** (4.5:1 for normal text) and **AAA** (7.0:1)

Litmus checks every `TermSpan` in every fixture against these thresholds. This catches issues that a palette-level check would miss — a theme might have good overall contrast but produce unreadable output in specific contexts (e.g. dim text on a colored background in `htop`).

## APCA Readability Scoring

In addition to WCAG ratios, litmus uses the Advanced Perceptual Contrast Algorithm (APCA) for readability scoring. APCA is more perceptually accurate than WCAG 2.1 contrast ratios — it accounts for the fact that dark text on a light background and light text on a dark background are not equally readable at the same ratio.

APCA scores are used in the readability filter on the browse page and in the per-fixture analysis on the detail page.

## Per-Fixture Contrast Analysis

On the theme detail and compare pages, contrast issues surface as interactive chips:

- Each chip represents a foreground/background pair that falls below the contrast threshold
- Click a chip to jump to the specific fixture and span where the issue occurs
- The fixture minimap in the sidebar shows issue badges, giving you a bird's-eye view of which fixtures have problems

This per-fixture analysis is only possible with simulated rendering — it requires access to the structured span data, not raster screenshots.

## CVD Simulation

`crates/litmus-model/src/cvd.rs` simulates color vision deficiency using Machado et al. 2009 transformation matrices. Three conditions are supported:

- **Protanopia** — reduced red sensitivity
- **Deuteranopia** — reduced green sensitivity
- **Tritanopia** — reduced blue sensitivity

The simulation:

1. Converts sRGB to linear RGB
2. Applies a 3×3 transformation matrix specific to the condition
3. Converts back to sRGB

The transform is applied at the `ProviderColors` level — it produces a new set of colors that flows through the existing rendering pipeline. This means contrast checking automatically works with simulated CVD colors too, so you can evaluate a theme's accessibility for users with color vision deficiency.

The CVD toggle in the sidebar switches between normal vision and the three simulation modes in real time.
