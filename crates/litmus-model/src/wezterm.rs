use serde::Deserialize;

use crate::error::ThemeError;
use crate::{AnsiColors, Color, Theme, defaults, parse_hex_color};

/// Raw TOML structure for a wezterm color scheme file.
#[derive(Debug, Deserialize)]
struct WeztermSchemeRaw {
    colors: WeztermColorsRaw,
    metadata: Option<WeztermMetadataRaw>,
}

#[derive(Debug, Deserialize)]
struct WeztermColorsRaw {
    background: String,
    foreground: String,
    cursor_bg: Option<String>,
    // cursor_fg and cursor_border are present in the format but not needed
    selection_bg: Option<String>,
    selection_fg: Option<String>,
    ansi: Vec<String>,
    brights: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct WeztermMetadataRaw {
    name: Option<String>,
}

/// Parse a wezterm color scheme TOML file into a `Theme`.
///
/// Wezterm format uses `ansi` (8 colors) and `brights` (8 colors) arrays
/// instead of named fields, and `cursor_bg`/`selection_bg` instead of
/// `cursor`/`selection_background`.
pub fn parse_wezterm_scheme(input: &str) -> Result<Theme, ThemeError> {
    let raw: WeztermSchemeRaw = toml::from_str(input)?;
    let c = &raw.colors;

    if c.ansi.len() != 8 {
        return Err(ThemeError::WrongColorCount(c.ansi.len()));
    }
    if c.brights.len() != 8 {
        return Err(ThemeError::WrongColorCount(c.brights.len()));
    }

    let background = parse_hex_color("background", &c.background)?;
    let foreground = parse_hex_color("foreground", &c.foreground)?;

    let cursor = c
        .cursor_bg
        .as_deref()
        .map(|v| parse_hex_color("cursor_bg", v))
        .transpose()?
        .unwrap_or_else(|| defaults::default_cursor(&foreground));

    let selection_background = c
        .selection_bg
        .as_deref()
        .map(|v| parse_hex_color("selection_bg", v))
        .transpose()?
        .unwrap_or_else(|| defaults::default_selection_bg(&background));

    let selection_foreground = c
        .selection_fg
        .as_deref()
        .map(|v| parse_hex_color("selection_fg", v))
        .transpose()?
        .unwrap_or_else(|| defaults::default_selection_fg(&foreground));

    // Build the 16-color array: ansi[0..8] + brights[0..8]
    let mut colors = Vec::with_capacity(16);
    for (i, hex) in c.ansi.iter().enumerate() {
        colors.push(parse_hex_color(&format!("ansi[{i}]"), hex)?);
    }
    for (i, hex) in c.brights.iter().enumerate() {
        colors.push(parse_hex_color(&format!("brights[{i}]"), hex)?);
    }

    let colors_array: [Color; 16] = colors
        .try_into()
        .map_err(|v: Vec<Color>| ThemeError::WrongColorCount(v.len()))?;

    let name = raw
        .metadata
        .and_then(|m| m.name)
        .unwrap_or_else(|| "Unnamed".to_string());

    Ok(Theme {
        name,
        foreground,
        background,
        cursor,
        selection_background,
        selection_foreground,
        ansi: AnsiColors::from_array(colors_array),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRUVBOX_DARK: &str = r##"
[colors]
ansi = [
    "#282828",
    "#cc241d",
    "#98971a",
    "#d79921",
    "#458588",
    "#b16286",
    "#689d6a",
    "#a89984",
]
background = "#282828"
brights = [
    "#928374",
    "#fb4934",
    "#b8bb26",
    "#fabd2f",
    "#83a598",
    "#d3869b",
    "#8ec07c",
    "#ebdbb2",
]
cursor_bg = "#ebdbb2"
cursor_border = "#ebdbb2"
cursor_fg = "#282828"
foreground = "#ebdbb2"
selection_bg = "#665c54"
selection_fg = "#ebdbb2"

[colors.indexed]

[metadata]
aliases = ["Gruvbox Dark (Gogh)"]
name = "GruvboxDark"
origin_url = "https://github.com/mbadolato/iTerm2-Color-Schemes"
wezterm_version = "20230320-124340-559cb7b0"
"##;

    #[test]
    fn parse_full_scheme() {
        let t = parse_wezterm_scheme(GRUVBOX_DARK).unwrap();
        assert_eq!(t.name, "GruvboxDark");
        assert_eq!(t.background, Color::new(0x28, 0x28, 0x28));
        assert_eq!(t.foreground, Color::new(0xeb, 0xdb, 0xb2));
        assert_eq!(t.cursor, Color::new(0xeb, 0xdb, 0xb2));
        assert_eq!(t.selection_background, Color::new(0x66, 0x5c, 0x54));
        assert_eq!(t.selection_foreground, Color::new(0xeb, 0xdb, 0xb2));
        // ANSI colors
        assert_eq!(t.ansi.black, Color::new(0x28, 0x28, 0x28));
        assert_eq!(t.ansi.red, Color::new(0xcc, 0x24, 0x1d));
        assert_eq!(t.ansi.bright_white, Color::new(0xeb, 0xdb, 0xb2));
    }

    #[test]
    fn parse_minimal_scheme() {
        let input = r##"
[colors]
ansi = ["#000000", "#cc0000", "#00cc00", "#cccc00", "#0000cc", "#cc00cc", "#00cccc", "#cccccc"]
background = "#000000"
brights = ["#555555", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff"]
foreground = "#ffffff"

[colors.indexed]

[metadata]
name = "Minimal"
"##;
        let t = parse_wezterm_scheme(input).unwrap();
        assert_eq!(t.name, "Minimal");
        // Defaults applied
        assert_eq!(t.cursor, t.foreground);
        assert_eq!(
            t.selection_background,
            Color::new(
                t.background.r.saturating_add(0x28),
                t.background.g.saturating_add(0x28),
                t.background.b.saturating_add(0x28),
            )
        );
    }

    #[test]
    fn wrong_ansi_count_errors() {
        let input = r##"
[colors]
ansi = ["#000000", "#cc0000"]
background = "#000000"
brights = ["#555555", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff"]
foreground = "#ffffff"

[colors.indexed]
"##;
        assert!(matches!(
            parse_wezterm_scheme(input),
            Err(ThemeError::WrongColorCount(2))
        ));
    }

    #[test]
    fn invalid_hex_errors() {
        let input = r##"
[colors]
ansi = ["#ZZZZZZ", "#cc0000", "#00cc00", "#cccc00", "#0000cc", "#cc00cc", "#00cccc", "#cccccc"]
background = "#000000"
brights = ["#555555", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff"]
foreground = "#ffffff"

[colors.indexed]
"##;
        assert!(matches!(
            parse_wezterm_scheme(input),
            Err(ThemeError::InvalidColor { .. })
        ));
    }
}
