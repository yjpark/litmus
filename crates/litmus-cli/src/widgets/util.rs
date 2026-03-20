pub fn to_ratatui_color(c: &litmus_model::Color) -> ratatui::style::Color {
    ratatui::style::Color::Rgb(c.r, c.g, c.b)
}
