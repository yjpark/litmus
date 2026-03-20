# Core Concepts

## The Three-Layer Theme Model

Terminal apps relate to themes in fundamentally different ways. Understanding this is key to the system's design.

### 1. Theme Providers

Apps that define a complete color palette independently. They are the "source of truth" for colors in their ecosystem.

Examples: kitty, wezterm, alacritty, neovim, helix

A provider's theme fully determines what you see — both for itself and for any consumer apps running inside it.

### 2. Theme Consumers

Apps that inherit colors from a provider. They use ANSI color codes or reference the provider's palette rather than defining their own.

Examples: `git diff`, `delta`, `ls --color`, `tig`, `bat`, `fd`, most CLI tools

A consumer **must be previewed within the context of a provider**. Showing `git diff` output alone is meaningless — it looks completely different under kitty+Tokyo Night vs kitty+Gruvbox.

### 3. Theme Silos (and Dual-Mode Apps)

Apps that define their own isolated theme, used only by themselves. Some apps can operate in both modes — e.g., `jjui` can use its own built-in theme (silo mode) or fall back to terminal ANSI colors (consumer mode).

The preview system should be able to show both modes for dual-mode apps.

## Provider Ecosystems

The provider/consumer relationship creates natural **ecosystems**:

- **Terminal ecosystem**: kitty (provider) → git diff, delta, tig, ls, bat, fd, jjui (consumers)
- **Editor ecosystem**: neovim (provider) → nvim-tree, telescope, lualine, which-key (consumers)
- **Editor ecosystem**: helix (provider) → (built-in UI elements as consumers)

A theme preview is most useful when it shows an entire ecosystem together — the provider plus its consumers rendering realistic content.
