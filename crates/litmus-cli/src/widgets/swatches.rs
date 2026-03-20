use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::theme_data::ThemeWithExtras;

pub struct SwatchesWidget<'a> {
    pub theme: &'a ThemeWithExtras,
}

const LABELS: [&str; 8] = ["blk", "red", "grn", "ylw", "blu", "mag", "cyn", "wht"];
const BRIGHT_SUFFIX: &str = "+";

fn to_ratatui_color(c: &litmus_model::Color) -> Color {
    Color::Rgb(c.r, c.g, c.b)
}

impl<'a> Widget for SwatchesWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 5 || area.width < 8 {
            return;
        }

        let swatch_width = (area.width / 8).max(4) as usize;
        let fg = to_ratatui_color(&self.theme.theme.foreground);
        let bg = to_ratatui_color(&self.theme.theme.background);
        let base_style = Style::default().fg(fg).bg(bg);
        let show_hex = swatch_width >= 9;

        // Fill background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_style(base_style).set_char(' ');
            }
        }

        let mut row = area.top();

        // Render normal (0-7) and bright (8-15) rows
        for pass in 0..2 {
            let offset = pass * 8;

            // Swatch row
            if row < area.bottom() {
                let mut col = area.left();
                for i in 0..8usize {
                    let color = to_ratatui_color(&self.theme.theme.colors[offset + i]);
                    let swatch = Span::styled(" ".repeat(swatch_width), Style::default().bg(color));
                    let line = Line::from(swatch);
                    let rect = Rect {
                        x: col,
                        y: row,
                        width: swatch_width as u16,
                        height: 1,
                    };
                    line.render(rect, buf);
                    col += swatch_width as u16;
                }
                row += 1;
            }

            // Label row
            if row < area.bottom() {
                let mut col = area.left();
                for i in 0..8usize {
                    let label = if pass == 0 {
                        LABELS[i].to_string()
                    } else {
                        format!("{}{}", LABELS[i], BRIGHT_SUFFIX)
                    };
                    let padded = center_pad(&label, swatch_width);
                    let span = Span::styled(padded, base_style);
                    let rect = Rect {
                        x: col,
                        y: row,
                        width: swatch_width as u16,
                        height: 1,
                    };
                    Line::from(span).render(rect, buf);
                    col += swatch_width as u16;
                }
                row += 1;
            }

            // Optional hex row
            if show_hex && row < area.bottom() {
                let mut col = area.left();
                for i in 0..8usize {
                    let hex = self.theme.theme.colors[offset + i].to_hex();
                    let padded = center_pad(&hex, swatch_width);
                    let span = Span::styled(padded, base_style);
                    let rect = Rect {
                        x: col,
                        y: row,
                        width: swatch_width as u16,
                        height: 1,
                    };
                    Line::from(span).render(rect, buf);
                    col += swatch_width as u16;
                }
                row += 1;
            }
        }

        // Blank separator
        row += 1;

        // Special colors row: fg, bg, cursor, sel
        if row < area.bottom() {
            let specials: &[(&str, &litmus_model::Color)] = &[
                ("fg", &self.theme.theme.foreground),
                ("bg", &self.theme.theme.background),
                ("cursor", &self.theme.cursor),
                ("sel", &self.theme.selection),
            ];

            let mut col = area.left();
            for (label, color) in specials {
                let swatch_color = to_ratatui_color(color);
                let text = format!("{} ", label);
                let label_span = Span::styled(text, base_style);
                let swatch_span = Span::styled("    ", Style::default().bg(swatch_color));
                let spacer = Span::styled("   ", base_style);
                let line = Line::from(vec![label_span, swatch_span, spacer]);
                let width = (label.len() + 1 + 4 + 3) as u16;
                let rect = Rect {
                    x: col,
                    y: row,
                    width: width.min(area.right() - col),
                    height: 1,
                };
                line.render(rect, buf);
                col += width;
                if col >= area.right() {
                    break;
                }
            }
        }
    }
}

fn center_pad(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }
    let total_pad = width - s.len();
    let left = total_pad / 2;
    let right = total_pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}
