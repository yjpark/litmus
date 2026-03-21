/// Known theme family prefixes (checked in order, longest match wins).
static FAMILIES: &[&str] = &[
    "Ayu",
    "Catppuccin",
    "Everforest",
    "Gruvbox",
    "Kanagawa",
    "Rose Pine",
    "Rosé Pine",
    "Solarized",
    "Tokyo Night",
];

/// Extract the family name from a theme name.
/// Returns the matched family prefix, or the full name for standalone themes.
pub fn theme_family(name: &str) -> &str {
    for &family in FAMILIES {
        if name.starts_with(family) {
            return family;
        }
    }
    // Standalone theme — family is the theme itself
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_families() {
        assert_eq!(theme_family("Catppuccin Mocha"), "Catppuccin");
        assert_eq!(theme_family("Catppuccin Frappe"), "Catppuccin");
        assert_eq!(theme_family("Tokyo Night"), "Tokyo Night");
        assert_eq!(theme_family("Tokyo Night Storm"), "Tokyo Night");
        assert_eq!(theme_family("Rose Pine Dawn"), "Rose Pine");
    }

    #[test]
    fn ayu_and_kanagawa_families() {
        assert_eq!(theme_family("Ayu Dark"), "Ayu");
        assert_eq!(theme_family("Ayu Light"), "Ayu");
        assert_eq!(theme_family("Kanagawa Wave"), "Kanagawa");
        assert_eq!(theme_family("Kanagawa Dragon"), "Kanagawa");
    }

    #[test]
    fn standalone_themes() {
        assert_eq!(theme_family("Dracula"), "Dracula");
        assert_eq!(theme_family("Nord"), "Nord");
    }
}
