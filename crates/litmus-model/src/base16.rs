use serde::Deserialize;

use crate::{AnsiColors, Color, Theme, defaults, error::ThemeError};

#[derive(Debug, Deserialize)]
struct Base16Raw {
    scheme: String,
    #[allow(dead_code)]
    author: Option<String>,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0a: String,
    #[serde(rename = "base0B")]
    base0b: String,
    #[serde(rename = "base0C")]
    base0c: String,
    #[serde(rename = "base0D")]
    base0d: String,
    #[serde(rename = "base0E")]
    base0e: String,
    #[serde(rename = "base0F")]
    base0f: String,
}

fn parse_field(field: &str, value: &str) -> Result<Color, ThemeError> {
    Color::from_hex(value).ok_or_else(|| ThemeError::InvalidColor {
        field: field.to_string(),
        value: value.to_string(),
    })
}

pub fn parse_base16_theme(input: &str) -> Result<Theme, ThemeError> {
    let raw: Base16Raw = serde_yml::from_str(input)?;

    let base00 = parse_field("base00", &raw.base00)?;
    let base01 = parse_field("base01", &raw.base01)?;
    let base02 = parse_field("base02", &raw.base02)?;
    let base03 = parse_field("base03", &raw.base03)?;
    let base04 = parse_field("base04", &raw.base04)?;
    let base05 = parse_field("base05", &raw.base05)?;
    let base06 = parse_field("base06", &raw.base06)?;
    let base07 = parse_field("base07", &raw.base07)?;
    let base08 = parse_field("base08", &raw.base08)?;
    let base09 = parse_field("base09", &raw.base09)?;
    let base0a = parse_field("base0A", &raw.base0a)?;
    let base0b = parse_field("base0B", &raw.base0b)?;
    let base0c = parse_field("base0C", &raw.base0c)?;
    let base0d = parse_field("base0D", &raw.base0d)?;
    let base0e = parse_field("base0E", &raw.base0e)?;
    let base0f = parse_field("base0F", &raw.base0f)?;

    let background = base00.clone();
    let foreground = base05.clone();

    // Standard base16-to-ANSI mapping
    let ansi = AnsiColors {
        black: base00.clone(),
        red: base08.clone(),
        green: base0b.clone(),
        yellow: base0a.clone(),
        blue: base0d.clone(),
        magenta: base0e.clone(),
        cyan: base0c.clone(),
        white: base05.clone(),
        bright_black: base03.clone(),
        bright_red: base09.clone(),
        bright_green: base01.clone(),
        bright_yellow: base02.clone(),
        bright_blue: base04.clone(),
        bright_magenta: base06.clone(),
        bright_cyan: base0f.clone(),
        bright_white: base07.clone(),
    };

    let cursor = defaults::default_cursor(&foreground);
    let selection_background = defaults::default_selection_bg(&background);
    let selection_foreground = defaults::default_selection_fg(&foreground);

    Ok(Theme {
        name: raw.scheme,
        background,
        foreground,
        cursor,
        selection_background,
        selection_foreground,
        ansi,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_BASE16: &str = r#"
scheme: "Tomorrow Night"
author: "Chris Kempson"
base00: "1d1f21"
base01: "282a2e"
base02: "373b41"
base03: "969896"
base04: "b4b7b4"
base05: "c5c8c6"
base06: "e0e0e0"
base07: "ffffff"
base08: "cc6666"
base09: "de935f"
base0A: "f0c674"
base0B: "b5bd68"
base0C: "8abeb7"
base0D: "81a2be"
base0E: "b294bb"
base0F: "a3685a"
"#;

    #[test]
    fn parse_valid_base16() {
        let t = parse_base16_theme(VALID_BASE16).unwrap();
        assert_eq!(t.name, "Tomorrow Night");
        assert_eq!(t.background, Color::new(0x1d, 0x1f, 0x21));
        assert_eq!(t.foreground, Color::new(0xc5, 0xc8, 0xc6));
        assert_eq!(t.ansi.red, Color::new(0xcc, 0x66, 0x66));
        assert_eq!(t.ansi.bright_white, Color::new(0xff, 0xff, 0xff));
    }

    #[test]
    fn bad_hex_returns_error() {
        let bad = VALID_BASE16.replace("cc6666", "zzzzzz");
        assert!(matches!(
            parse_base16_theme(&bad),
            Err(ThemeError::InvalidColor { field, .. }) if field == "base08"
        ));
    }

    #[test]
    fn missing_field_returns_error() {
        let missing = VALID_BASE16
            .lines()
            .filter(|l| !l.contains("base0F"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(matches!(parse_base16_theme(&missing), Err(ThemeError::Yaml(_))));
    }
}
