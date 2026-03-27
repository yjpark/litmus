# Consumers

A **consumer** is an app that inherits colors from its provider. It doesn't define its own palette — it references colors by their ANSI index. What those colors actually look like depends entirely on the provider's theme.

## How Consumers Use Color

When `git diff` displays an addition in green, it emits ANSI escape code 32 (foreground color 2). The terminal emulator — the provider — maps color index 2 to an RGB value from its current theme. The consumer never knows what "green" actually looks like.

This is why the same `git diff` output looks completely different under Tokyo Night (muted teal green) vs Gruvbox (warm olive green) vs Catppuccin Mocha (pastel mint). The diff is identical; the provider's palette is different.

## Consumers in Litmus

Litmus represents consumers through **fixtures** — reproducible terminal scenarios that exercise specific tools. Each fixture runs a real command (`git diff`, `cargo build`, `bat`, etc.) and captures the output.

| Fixture | Consumer | What it demonstrates |
|---------|----------|---------------------|
| git-diff | `git diff` | Diff colors: additions, deletions, context |
| cargo-build | `cargo build` | Compiler output: warnings, errors, notes |
| bat-syntax | `bat` | Syntax-highlighted source with line numbers |
| ripgrep-search | `rg` | Match highlighting, filenames, line numbers |
| ls-color | `ls --color` | File type colors: dirs, executables, symlinks |
| git-log | `git log --graph` | Graph colors and branch decorations |
| shell-prompt | bash session | Prompt colors and command output |
| python-repl | `python3` | REPL output, tracebacks |
| htop | `top` | CPU, memory, process table |
| log-viewer | app logs | Structured logs: INFO/WARN/ERROR/DEBUG |
| color-swatch | ANSI palette | Reference palette: 16 ANSI + 256-color |
| color-showcase | CI dashboard | Simulated deploy pipeline using all 16 ANSI colors |
| editor-ui | text editor | Syntax highlighting, status bar, line numbers |

Each fixture is captured once per (provider, theme) combination, giving you the exact pixels your terminal would display.

## Why Swatches Aren't Enough

A 16-color swatch tells you the palette. It doesn't tell you:

- Whether `git diff` additions are distinguishable from context lines
- Whether cargo warnings are readable against the background
- Whether `bat` line numbers have enough contrast
- Whether `ls` directory colors clash with executable colors

These are consumer-specific questions that depend on how each app maps ANSI indices to semantic meaning. Litmus answers them by showing the actual consumer output, not abstract swatches.
