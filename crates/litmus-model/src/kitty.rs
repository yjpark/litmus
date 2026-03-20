use crate::{Color, Theme};

pub struct KittyTheme {
    pub theme: Theme,
    pub cursor: Option<Color>,
    pub selection_background: Option<Color>,
    pub selection_foreground: Option<Color>,
}

pub fn parse_kitty_theme(input: &str) -> Option<KittyTheme> {
    let mut name: Option<String> = None;
    let mut foreground: Option<Color> = None;
    let mut background: Option<Color> = None;
    let mut colors: [Option<Color>; 16] = Default::default();
    let mut cursor: Option<Color> = None;
    let mut selection_background: Option<Color> = None;
    let mut selection_foreground: Option<Color> = None;

    for line in input.lines() {
        let line = line.trim();

        // Extract theme name from metadata comment: ## name: <value>
        if let Some(rest) = line.strip_prefix("## ") {
            if let Some(val) = rest.strip_prefix("name:") {
                name = Some(val.trim().to_string());
            }
            continue;
        }

        // Skip blank lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, val) = match line.split_once(char::is_whitespace) {
            Some(pair) => pair,
            None => continue,
        };
        let val = val.trim();

        match key {
            "foreground" => foreground = Color::from_hex(val),
            "background" => background = Color::from_hex(val),
            "cursor" => cursor = Color::from_hex(val),
            "selection_background" => selection_background = Color::from_hex(val),
            "selection_foreground" => selection_foreground = Color::from_hex(val),
            k if k.starts_with("color") => {
                if let Ok(idx) = k["color".len()..].parse::<usize>()
                    && idx < 16
                {
                    colors[idx] = Color::from_hex(val);
                }
            }
            _ => {}
        }
    }

    let foreground = foreground?;
    let background = background?;
    let colors: Option<Vec<Color>> = colors.into_iter().collect();
    let colors = colors?;

    Some(KittyTheme {
        theme: Theme {
            name: name.unwrap_or_else(|| "Unnamed".to_string()),
            foreground,
            background,
            colors,
        },
        cursor,
        selection_background,
        selection_foreground,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_THEME: &str = r#"
## name: Test Theme

background #1a1b26
foreground #a9b1d6
cursor #c0caf5
selection_background #283d5c
selection_foreground #ffffff

color0  #151622
color1  #f7768e
color2  #9ece6a
color3  #e0af68
color4  #7aa2f7
color5  #bb9af7
color6  #7dcfff
color7  #a9b1d6
color8  #414468
color9  #f7768e
color10 #9ece6a
color11 #e0af68
color12 #7aa2f7
color13 #bb9af7
color14 #7dcfff
color15 #c0caf5
"#;

    #[test]
    fn parse_complete_theme() {
        let kt = parse_kitty_theme(FULL_THEME).unwrap();
        assert_eq!(kt.theme.name, "Test Theme");
        assert_eq!(kt.theme.background, Color::new(0x1a, 0x1b, 0x26));
        assert_eq!(kt.theme.foreground, Color::new(0xa9, 0xb1, 0xd6));
        assert_eq!(kt.theme.colors.len(), 16);
        assert_eq!(kt.theme.colors[0], Color::new(0x15, 0x16, 0x22));
        assert_eq!(kt.theme.colors[15], Color::new(0xc0, 0xca, 0xf5));
        assert_eq!(kt.cursor, Some(Color::new(0xc0, 0xca, 0xf5)));
        assert_eq!(kt.selection_background, Some(Color::new(0x28, 0x3d, 0x5c)));
        assert_eq!(kt.selection_foreground, Some(Color::new(0xff, 0xff, 0xff)));
    }

    #[test]
    fn parse_without_optional_fields() {
        let input = FULL_THEME
            .lines()
            .filter(|l| !l.trim_start().starts_with("cursor") && !l.trim_start().starts_with("selection"))
            .collect::<Vec<_>>()
            .join("\n");
        let kt = parse_kitty_theme(&input).unwrap();
        assert!(kt.cursor.is_none());
        assert!(kt.selection_background.is_none());
        assert!(kt.selection_foreground.is_none());
    }

    #[test]
    fn missing_required_fields_returns_none() {
        // Missing foreground
        let input = "background #1a1b26\ncolor0 #000000\n";
        assert!(parse_kitty_theme(input).is_none());

        // Missing background
        let input = "foreground #a9b1d6\ncolor0 #000000\n";
        assert!(parse_kitty_theme(input).is_none());

        // Missing colors (only 15)
        let mut lines = vec![
            "foreground #a9b1d6".to_string(),
            "background #1a1b26".to_string(),
        ];
        for i in 0..15 {
            lines.push(format!("color{} #000000", i));
        }
        assert!(parse_kitty_theme(&lines.join("\n")).is_none());
    }

    #[test]
    fn color_from_hex_edge_cases() {
        assert_eq!(Color::from_hex("#ff8800"), Some(Color::new(0xff, 0x88, 0x00)));
        assert_eq!(Color::from_hex("ff8800"), Some(Color::new(0xff, 0x88, 0x00)));
        assert_eq!(Color::from_hex("#FF8800"), Some(Color::new(0xff, 0x88, 0x00)));
        assert_eq!(Color::from_hex("#ff88"), None);
        assert_eq!(Color::from_hex(""), None);
        assert_eq!(Color::from_hex("#gghhii"), None);
    }
}
