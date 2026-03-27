# Rendering

Litmus displays theme previews in two modes: real screenshots and simulated rendering. Both start from the same data — the difference is whether you're looking at captured pixels or reconstructed output.

## The TermOutput Model

The simulated rendering path uses `TermOutput`, defined in `crates/litmus-model/src/term_output.rs`. This replaced an earlier handcrafted scene system with a model that faithfully represents real terminal output.

### TermColor

```rust
enum TermColor {
    Default,        // theme foreground or background (context-dependent)
    Ansi(u8),       // 0–15, resolved from provider's ANSI palette
    Indexed(u8),    // 16–255, fixed xterm-256 color palette
    Rgb(u8, u8, u8) // 24-bit truecolor, used as-is
}
```

`TermColor` covers the full terminal color space. ANSI colors (0–15) are theme-dependent — they resolve differently per provider and theme. Indexed colors (16–255) and RGB colors are fixed regardless of theme.

### TermSpan

```rust
struct TermSpan {
    text: String,
    fg: TermColor,
    bg: TermColor,
    bold: bool,
    italic: bool,
    dim: bool,
    underline: bool,
}
```

A span is a run of text with uniform styling. A line of terminal output is a vector of spans.

### Color Resolution

`TermColor::resolve()` takes a `ProviderColors` (the 21-color palette from a provider's theme: 16 ANSI + fg/bg/cursor/selection_bg/selection_fg) and maps semantic references to concrete RGB:

- `Default` → provider's foreground or background color
- `Ansi(n)` → the nth color in the provider's palette
- `Indexed(n)` → looked up from the fixed xterm-256 color table
- `Rgb(r,g,b)` → passed through unchanged

This indirection is what makes a single fixture definition render correctly across every theme. The fixture data says "use ANSI color 2 for additions" — what that looks like depends on the provider.

## Dual Display

The theme detail page shows both rendering modes for each fixture:

- **Screenshot** — the WebP image captured from a real terminal, loaded from R2. This is the ground truth — what you'll actually see.
- **Simulated** — the fixture's `output.json` (parsed ANSI) rendered as styled HTML. Each `TermSpan` becomes a `<span>` with inline `color` and `background-color` CSS properties resolved from the current theme.

Simulated rendering exists because raster screenshots can't support:

- **CVD simulation** — transforming colors to show how the theme appears under color vision deficiency requires access to the individual color values, not pixels
- **Contrast analysis** — checking every foreground/background pair against WCAG thresholds requires the structured span data
- **Theme switching without re-capture** — simulated rendering updates instantly when you change themes; screenshots require a full capture cycle

## Web Renderer

`crates/litmus-web/src/term_renderer.rs` converts `TermOutput` to Dioxus HTML. For each span:

1. Resolve `fg` and `bg` via `TermColor::resolve()` with the active `ProviderColors`
2. Apply bold, italic, dim as CSS `font-weight`, `font-style`, `opacity`
3. Emit a `<span>` with inline styles inside a `<pre>` block

The result is a monospace rendering that closely matches the real terminal output, minus font-specific details like ligatures and glyph spacing.
