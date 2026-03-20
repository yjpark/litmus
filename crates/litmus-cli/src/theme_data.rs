use litmus_model::{Color, Theme};

pub struct ThemeWithExtras {
    pub theme: Theme,
    pub cursor: Color,
    pub selection: Color,
}

pub fn tokyo_night() -> ThemeWithExtras {
    ThemeWithExtras {
        theme: Theme {
            name: "Tokyo Night".to_string(),
            background: Color::new(0x1a, 0x1b, 0x26),
            foreground: Color::new(0xa9, 0xb1, 0xd6),
            colors: vec![
                // Normal (0-7)
                Color::new(0x15, 0x16, 0x22), // black
                Color::new(0xf7, 0x76, 0x8e), // red
                Color::new(0x9e, 0xce, 0x6a), // green
                Color::new(0xe0, 0xaf, 0x68), // yellow
                Color::new(0x7a, 0xa2, 0xf7), // blue
                Color::new(0xbb, 0x9a, 0xf7), // magenta
                Color::new(0x7d, 0xcf, 0xff), // cyan
                Color::new(0xa9, 0xb1, 0xd6), // white
                // Bright (8-15)
                Color::new(0x41, 0x44, 0x68), // bright black
                Color::new(0xf7, 0x76, 0x8e), // bright red
                Color::new(0x9e, 0xce, 0x6a), // bright green
                Color::new(0xe0, 0xaf, 0x68), // bright yellow
                Color::new(0x7a, 0xa2, 0xf7), // bright blue
                Color::new(0xbb, 0x9a, 0xf7), // bright magenta
                Color::new(0x7d, 0xcf, 0xff), // bright cyan
                Color::new(0xc0, 0xca, 0xf5), // bright white
            ],
        },
        cursor: Color::new(0xc0, 0xca, 0xf5),
        selection: Color::new(0x28, 0x3d, 0x5c),
    }
}
