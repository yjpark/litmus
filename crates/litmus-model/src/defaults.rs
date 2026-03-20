use crate::Color;

pub fn default_cursor(fg: &Color) -> Color {
    fg.clone()
}

pub fn default_selection_bg(bg: &Color) -> Color {
    Color::new(
        bg.r.saturating_add(0x28),
        bg.g.saturating_add(0x28),
        bg.b.saturating_add(0x28),
    )
}

pub fn default_selection_fg(fg: &Color) -> Color {
    fg.clone()
}
