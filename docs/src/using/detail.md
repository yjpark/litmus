# Theme Detail

The detail page shows a single theme across all 13 fixtures. This is where you evaluate whether a theme works for your workflow.

## Dual Display

Each fixture is shown in two modes:

- **Screenshot** — a real capture from a headless terminal emulator, showing exactly what you'd see. These are served from a CDN and lazy-loaded as you scroll.
- **Simulated** — the same terminal output rendered from parsed ANSI codes. This enables features that can't work on raster images: contrast analysis and CVD simulation.

## Contrast Analysis

Below the header, contrast issue chips summarize readability problems found across all fixtures. Click a chip to jump to the specific fixture and span where the issue occurs. Issues are identified by checking every foreground/background color pair against WCAG contrast thresholds.

## Fixture Minimap

The sidebar shows a visual minimap of all 13 fixtures. Click any fixture to scroll directly to it. Fixtures with contrast issues display a badge indicating the number of problems.

## CVD Simulation

The sidebar includes a color vision deficiency simulation toggle. Switch between normal vision, protanopia, deuteranopia, and tritanopia to see how the theme appears under different vision conditions. This applies to the simulated rendering — it transforms the entire color palette in real time.
