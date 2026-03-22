use litmus_model::Theme;

/// Embedded theme data for WASM builds.
static THEME_DATA: &[&str] = &[
    // Andromeda
    include_str!("../../../themes/andromeda.toml"),
    // Ayu
    include_str!("../../../themes/ayu/ayu-dark.toml"),
    include_str!("../../../themes/ayu/ayu-light.toml"),
    // Catppuccin
    include_str!("../../../themes/catppuccin/frappe.toml"),
    include_str!("../../../themes/catppuccin/latte.toml"),
    include_str!("../../../themes/catppuccin/macchiato.toml"),
    include_str!("../../../themes/catppuccin/mocha.toml"),
    // Cyberdream
    include_str!("../../../themes/cyberdream/cyberdream-dark.toml"),
    include_str!("../../../themes/cyberdream/cyberdream-light.toml"),
    // Dracula
    include_str!("../../../themes/dracula.toml"),
    // Everforest
    include_str!("../../../themes/everforest/everforest-dark.toml"),
    include_str!("../../../themes/everforest/everforest-light.toml"),
    // Flexoki
    include_str!("../../../themes/flexoki/flexoki-dark.toml"),
    include_str!("../../../themes/flexoki/flexoki-light.toml"),
    // GitHub
    include_str!("../../../themes/github/github-dark.toml"),
    include_str!("../../../themes/github/github-dark-dimmed.toml"),
    include_str!("../../../themes/github/github-light.toml"),
    // Gruvbox
    include_str!("../../../themes/gruvbox/gruvbox-dark.toml"),
    include_str!("../../../themes/gruvbox/gruvbox-light.toml"),
    // Horizon
    include_str!("../../../themes/horizon.toml"),
    // Iceberg
    include_str!("../../../themes/iceberg/iceberg-dark.toml"),
    include_str!("../../../themes/iceberg/iceberg-light.toml"),
    // Kanagawa
    include_str!("../../../themes/kanagawa/kanagawa-dragon.toml"),
    include_str!("../../../themes/kanagawa/kanagawa-wave.toml"),
    // Material
    include_str!("../../../themes/material.toml"),
    // Melange
    include_str!("../../../themes/melange/melange-dark.toml"),
    include_str!("../../../themes/melange/melange-light.toml"),
    // Modus
    include_str!("../../../themes/modus/modus-operandi.toml"),
    include_str!("../../../themes/modus/modus-vivendi.toml"),
    // Monokai
    include_str!("../../../themes/monokai.toml"),
    // Moonlight
    include_str!("../../../themes/moonlight.toml"),
    // Night Owl
    include_str!("../../../themes/night-owl/light-owl.toml"),
    include_str!("../../../themes/night-owl/night-owl.toml"),
    // Nightfox
    include_str!("../../../themes/nightfox/dawnfox.toml"),
    include_str!("../../../themes/nightfox/dayfox.toml"),
    include_str!("../../../themes/nightfox/duskfox.toml"),
    include_str!("../../../themes/nightfox/nightfox.toml"),
    include_str!("../../../themes/nightfox/nordfox.toml"),
    include_str!("../../../themes/nightfox/terafox.toml"),
    // Nord
    include_str!("../../../themes/nord.toml"),
    // One
    include_str!("../../../themes/one-dark.toml"),
    include_str!("../../../themes/one-light.toml"),
    // Oxocarbon
    include_str!("../../../themes/oxocarbon/oxocarbon-dark.toml"),
    include_str!("../../../themes/oxocarbon/oxocarbon-light.toml"),
    // Palenight
    include_str!("../../../themes/palenight.toml"),
    // Poimandres
    include_str!("../../../themes/poimandres.toml"),
    // Rose Pine
    include_str!("../../../themes/rose-pine/rose-pine.toml"),
    include_str!("../../../themes/rose-pine/rose-pine-dawn.toml"),
    include_str!("../../../themes/rose-pine/rose-pine-moon.toml"),
    // Snazzy
    include_str!("../../../themes/snazzy.toml"),
    // Solarized
    include_str!("../../../themes/solarized/solarized-dark.toml"),
    include_str!("../../../themes/solarized/solarized-light.toml"),
    // Sonokai
    include_str!("../../../themes/sonokai/sonokai.toml"),
    include_str!("../../../themes/sonokai/sonokai-shusia.toml"),
    // Tender
    include_str!("../../../themes/tender.toml"),
    // Tokyo Night
    include_str!("../../../themes/tokyo-night/tokyo-night.toml"),
    include_str!("../../../themes/tokyo-night/tokyo-night-day.toml"),
    include_str!("../../../themes/tokyo-night/tokyo-night-storm.toml"),
    // Vesper
    include_str!("../../../themes/vesper.toml"),
    // Zenburn
    include_str!("../../../themes/zenburn.toml"),
];

/// Load all embedded themes. Returns themes sorted by name.
pub fn load_embedded_themes() -> Vec<Theme> {
    let mut themes: Vec<Theme> = THEME_DATA
        .iter()
        .filter_map(|toml_str| litmus_model::toml_format::parse_toml_theme(toml_str).ok())
        .collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}
