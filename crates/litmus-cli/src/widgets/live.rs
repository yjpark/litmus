use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use litmus_model::Theme;
use super::util::to_ratatui_color;

pub struct LiveWidget<'a> {
    pub theme: &'a Theme,
    pub git_diff: &'a [String],
    pub ls_output: &'a [String],
}

impl Widget for LiveWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let t = self.theme;
        let fg = to_ratatui_color(&t.foreground);
        let bg = to_ratatui_color(&t.background);
        let base = Style::default().fg(fg).bg(bg);
        let ansi = t.ansi.as_array();
        let c = |idx: usize| to_ratatui_color(ansi[idx]);

        // Fill background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_style(base).set_char(' ');
            }
        }

        let prompt = |cmd: &str| -> Line<'static> {
            Line::from(vec![
                Span::styled("user@host", Style::default().fg(c(10)).bg(bg)),
                Span::styled(" ", base),
                Span::styled("~/projects/myapp", Style::default().fg(c(12)).bg(bg)),
                Span::styled(" ", base),
                Span::styled("(main)", Style::default().fg(c(13)).bg(bg)),
                Span::styled(format!(" $ {cmd}"), base),
            ])
        };

        let mut lines: Vec<Line<'static>> = Vec::new();

        // Git diff section
        lines.push(prompt("git diff"));
        lines.push(Line::from(""));
        if self.git_diff.is_empty() {
            lines.push(Line::from(Span::styled(
                "(no changes)",
                Style::default().fg(c(8)).bg(bg),
            )));
        } else {
            for raw in self.git_diff {
                lines.push(style_diff_line(raw, base, bg, &c));
            }
        }
        lines.push(Line::from(""));

        // ls -la section
        lines.push(prompt("ls -la"));
        if self.ls_output.is_empty() {
            lines.push(Line::from(Span::styled(
                "(no output)",
                Style::default().fg(c(8)).bg(bg),
            )));
        } else {
            for raw in self.ls_output {
                lines.push(style_ls_line(raw, base, bg, &c));
            }
        }

        // Render lines into buffer
        for (i, line) in lines.iter().enumerate() {
            let y = area.top() + i as u16;
            if y >= area.bottom() {
                break;
            }
            let rect = Rect { x: area.left(), y, width: area.width, height: 1 };
            line.clone().render(rect, buf);
        }
    }
}

fn style_diff_line(
    raw: &str,
    base: Style,
    bg: ratatui::style::Color,
    c: &impl Fn(usize) -> ratatui::style::Color,
) -> Line<'static> {
    if raw.starts_with("diff ")
        || raw.starts_with("index ")
        || raw.starts_with("--- ")
        || raw.starts_with("+++ ")
    {
        Line::from(Span::styled(
            raw.to_owned(),
            base.add_modifier(Modifier::BOLD),
        ))
    } else if raw.starts_with("@@") {
        Line::from(Span::styled(
            raw.to_owned(),
            Style::default().fg(c(6)).bg(bg),
        ))
    } else if raw.starts_with('+') {
        Line::from(Span::styled(
            raw.to_owned(),
            Style::default().fg(c(2)).bg(bg),
        ))
    } else if raw.starts_with('-') {
        Line::from(Span::styled(
            raw.to_owned(),
            Style::default().fg(c(1)).bg(bg),
        ))
    } else {
        Line::from(Span::styled(raw.to_owned(), base))
    }
}

fn style_ls_line(
    raw: &str,
    base: Style,
    bg: ratatui::style::Color,
    c: &impl Fn(usize) -> ratatui::style::Color,
) -> Line<'static> {
    if raw.is_empty() || raw.starts_with("total") {
        return Line::from(Span::styled(raw.to_owned(), base));
    }

    let first_char = raw.chars().next().unwrap_or(' ');
    let perms = raw.split_whitespace().next().unwrap_or("");

    // Find where the filename starts (9th whitespace-separated field)
    let mut fields = 0usize;
    let mut name_start = raw.len();
    let mut in_ws = true;
    for (i, ch) in raw.char_indices() {
        if ch.is_ascii_whitespace() {
            in_ws = true;
        } else if in_ws {
            in_ws = false;
            fields += 1;
            if fields == 9 {
                name_start = i;
                break;
            }
        }
    }

    let meta = raw[..name_start].to_owned();
    let name = raw[name_start..].to_owned();

    let name_style = if first_char == 'd' {
        Style::default().fg(c(12)).bg(bg)
    } else if first_char == 'l' {
        Style::default().fg(c(14)).bg(bg)
    } else if perms.contains('x') {
        Style::default().fg(c(10)).bg(bg)
    } else if name.starts_with('.') {
        Style::default().fg(c(8)).bg(bg)
    } else {
        base
    };

    Line::from(vec![
        Span::styled(meta, base),
        Span::styled(name, name_style),
    ])
}
