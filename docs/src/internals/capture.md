# Capture Pipeline

Litmus captures real screenshots from real terminal emulators. This is the system that makes litmus's previews trustworthy — you see the actual pixels your terminal would render, not a simulation.

## Why Real Captures

Simulated rendering can reproduce colors accurately, but it can't capture everything a terminal emulator does: font rendering, ligatures, cursor styles, padding, actual app behavior. A real screenshot of `git diff` running in kitty with Tokyo Night is the definitive answer to "what will this look like."

Litmus uses simulated rendering too (for contrast analysis and CVD simulation), but the screenshots are the ground truth.

## The `litmus-capture` Crate

The capture pipeline lives in `crates/litmus-capture/`. It's a native binary (not WASM) that requires a headless Wayland compositor with GPU access.

### Capture Flow

For each (provider, theme, fixture) combination:

1. **Configure** — write a temporary provider config file with the theme's colors and a fixed terminal geometry (80 columns × 32 rows, 12pt FiraCode, 1280×960px)
2. **Setup** — run the fixture's `setup.sh` to create any required state (git repos, source files, etc.) in a temp directory
3. **Launch** — start the terminal emulator headlessly with the fixture's `command.sh`
4. **Capture** — wait for the command to finish, take a screenshot with `grim`
5. **Convert** — PNG → WebP for smaller file sizes
6. **Checksum** — compute SHA-256 for cache-busting

The high-resolution capture gives crisp text when displayed at smaller sizes in the web app.

### ANSI Parsing

`litmus-capture` also parses raw ANSI escape sequences from terminal command output into structured data. This produces `TermOutput` — the same format used for simulated rendering in the web app. The parser handles:

- Standard ANSI color codes (16 colors)
- 256-color extended palette
- 24-bit truecolor (RGB)
- SGR attributes: bold, italic, dim, underline, inverse

Each fixture's parsed output is saved as `output.json` alongside its scripts.

### Batch Capture

`capture-all` runs captures for every combination of provider × theme × fixture in parallel, using half the available CPU cores. With 2 providers × 58 themes × 13 fixtures = ~1,500 screenshots per full run.

## Manifest and CDN

Screenshots are stored in a Cloudflare R2 bucket. A `manifest.json` file indexes every screenshot with metadata:

```json
{
  "providers": [{"slug": "kitty", "name": "Kitty", "version": "..."}],
  "fixtures": [{"id": "git-diff", "name": "Git Diff", "description": "..."}],
  "screenshots": [{
    "provider": "kitty",
    "theme": "tokyo-night",
    "fixture": "git-diff",
    "width": 1280,
    "height": 960,
    "url": "kitty/tokyo-night/git-diff.webp",
    "format": "webp",
    "sha256": "..."
  }]
}
```

The web app fetches the manifest once at startup, then lazy-loads individual screenshots as the user scrolls. URLs include a checksum query parameter (`?v=abc12345`) for cache-busting, allowing aggressive caching: 1-year TTL on images, 1-minute TTL on the manifest.

## Color Extraction

`litmus extract-colors` reads each provider's vendored theme data (e.g. kitty's `themes/` directory, wezterm's built-in theme registry) and writes `.kitty.toml` / `.wezterm.toml` files with the resolved RGB palette. This is how litmus gets the actual colors each provider uses — not a hand-transcribed approximation, but the values parsed from the provider's own theme definitions.
