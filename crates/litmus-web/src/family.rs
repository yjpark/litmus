/// Known theme family prefixes (checked in order, longest match wins).
static FAMILIES: &[&str] = &[
    "Ayu",
    "Catppuccin",
    "Cyberdream",
    "Everforest",
    "Flexoki",
    "GitHub",
    "Gruvbox",
    "Iceberg",
    "Kanagawa",
    "Melange",
    "Modus",
    "One",
    "Oxocarbon",
    "Rose Pine",
    "Rosé Pine",
    "Solarized",
    "Sonokai",
    "Tokyo Night",
];

/// Themes grouped by suffix or explicit membership rather than prefix.
static SUFFIX_FAMILIES: &[(&str, &[&str])] = &[
    ("Nightfox", &["Dawnfox", "Dayfox", "Duskfox", "Nightfox", "Nordfox", "Terafox"]),
    ("Owl", &["Night Owl", "Light Owl"]),
];

/// Extract the family name from a theme name.
/// Returns the matched family prefix, or the full name for standalone themes.
pub fn theme_family(name: &str) -> &str {
    for &family in FAMILIES {
        if name.starts_with(family) {
            return family;
        }
    }
    for &(family, members) in SUFFIX_FAMILIES {
        if members.contains(&name) {
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

    #[test]
    fn new_prefix_families() {
        assert_eq!(theme_family("GitHub Dark"), "GitHub");
        assert_eq!(theme_family("GitHub Light"), "GitHub");
        assert_eq!(theme_family("GitHub Dark Dimmed"), "GitHub");
        assert_eq!(theme_family("Flexoki Dark"), "Flexoki");
        assert_eq!(theme_family("Flexoki Light"), "Flexoki");
        assert_eq!(theme_family("Cyberdream Dark"), "Cyberdream");
        assert_eq!(theme_family("Modus Vivendi"), "Modus");
        assert_eq!(theme_family("Modus Operandi"), "Modus");
        assert_eq!(theme_family("Oxocarbon Dark"), "Oxocarbon");
        assert_eq!(theme_family("Melange Dark"), "Melange");
        assert_eq!(theme_family("One Dark"), "One");
        assert_eq!(theme_family("One Light"), "One");
        assert_eq!(theme_family("Sonokai Shusia"), "Sonokai");
        assert_eq!(theme_family("Iceberg Dark"), "Iceberg");
    }

    #[test]
    fn suffix_families() {
        assert_eq!(theme_family("Nightfox"), "Nightfox");
        assert_eq!(theme_family("Dawnfox"), "Nightfox");
        assert_eq!(theme_family("Dayfox"), "Nightfox");
        assert_eq!(theme_family("Duskfox"), "Nightfox");
        assert_eq!(theme_family("Nordfox"), "Nightfox");
        assert_eq!(theme_family("Terafox"), "Nightfox");
        assert_eq!(theme_family("Night Owl"), "Owl");
        assert_eq!(theme_family("Light Owl"), "Owl");
    }
}
