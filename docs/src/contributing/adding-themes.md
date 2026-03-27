# Adding Themes

Each theme in litmus consists of a **theme definition** file and one or more **provider color files**.

## 1. Create the Theme Definition

Theme definitions live in `themes/`. Standalone themes go directly in the directory; family members go in a subdirectory:

```
themes/dracula.toml              # standalone
themes/catppuccin/mocha.toml     # family member
```

A theme definition is minimal — just a name, variant, and provider mappings:

```toml
name = "Dracula"
variant = "dark"

[providers]
kitty = "Dracula"
wezterm = "Dracula (Gogh)"
```

- **name** — human-readable display name
- **variant** — `"dark"` or `"light"`
- **providers** — maps each supported provider to the theme name in that provider's theme registry. Not every theme needs to support every provider.

## 2. Extract Provider Colors

Provider color files are auto-generated — don't write them by hand. Run:

```bash
# Extract colors for all themes from all providers
mise run extract-colors
```

This reads each provider's vendored theme data and writes `.kitty.toml` / `.wezterm.toml` files with the resolved RGB palette:

```
themes/catppuccin/mocha.kitty.toml      # auto-generated
themes/catppuccin/mocha.wezterm.toml    # auto-generated
```

If your theme's provider name doesn't match anything in the vendored data, the extraction will skip it — check that the `[providers]` mapping matches the provider's actual theme registry names.

## 3. Register in `themes.rs`

Open `crates/litmus-web/src/themes.rs` and add entries to both arrays.

In `DEFINITION_DATA`:

```rust
("dracula", include_str!("../../../themes/dracula.toml")),
```

In `PROVIDER_COLORS_DATA` (one entry per provider color file):

```rust
("dracula", include_str!("../../../themes/dracula.kitty.toml")),
("dracula", include_str!("../../../themes/dracula.wezterm.toml")),
```

The first element is the theme slug (must match the slug used in the definition entry). Keep both arrays sorted alphabetically.

## 4. Register the Family (if new)

If your theme belongs to a family not yet registered, add the family prefix in `crates/litmus-web/src/family.rs`:

```rust
static FAMILIES: &[&str] = &[
    "Ayu",
    "Catppuccin",
    // ... add your family name here ...
];
```

Family grouping uses prefix matching — "Catppuccin Mocha" matches the "Catppuccin" prefix.

## 5. Capture Screenshots

After adding the theme, capture screenshots for it:

```bash
mise run capture-kitty    # if kitty provider is mapped
mise run capture-wezterm  # if wezterm provider is mapped
```

## 6. Verify

1. Check `.bacon-claude-diagnostics` for compilation errors
2. Start the web app with `mise run dev`
3. Confirm the theme appears in the theme list
4. Check that all fixtures render on the detail page
5. Verify family grouping if applicable
