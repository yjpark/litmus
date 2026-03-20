use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::theme_data::ThemeWithExtras;
use super::util::to_ratatui_color;

pub struct MockupsWidget<'a> {
    pub theme: &'a ThemeWithExtras,
}

impl<'a> Widget for MockupsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let t = &self.theme.theme;
        let fg = to_ratatui_color(&t.foreground);
        let bg = to_ratatui_color(&t.background);
        let base = Style::default().fg(fg).bg(bg);

        // Fill background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_style(base).set_char(' ');
            }
        }

        let c = |idx: usize| to_ratatui_color(&t.colors[idx]);

        // Prompt spans helper
        let prompt_spans = || -> Vec<Span<'static>> {
            vec![
                Span::styled("user@host", Style::default().fg(c(10)).bg(bg)),
                Span::styled(" ", base),
                Span::styled("~/projects/myapp", Style::default().fg(c(12)).bg(bg)),
                Span::styled(" ", base),
                Span::styled("(main)", Style::default().fg(c(13)).bg(bg)),
                Span::styled(" $ ", base),
            ]
        };

        let lines: Vec<Line<'static>> = vec![
            // --- Section 1: shell prompt + git diff command ---
            {
                let mut spans = prompt_spans();
                spans.push(Span::styled("git diff", base));
                Line::from(spans)
            },
            Line::from(""),
            // --- Section 2: git diff output ---
            Line::from(vec![
                Span::styled(
                    "diff --git a/src/main.rs b/src/main.rs",
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "index 3f4a1b2..8c9d0e1 100644",
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "--- a/src/main.rs",
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "+++ b/src/main.rs",
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "@@ -12,7 +12,9 @@ fn run(terminal: &mut Terminal<...>) -> Result<()> {",
                    Style::default().fg(c(6)).bg(bg),
                ),
            ]),
            Line::from(vec![Span::styled(
                " fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {",
                base,
            )]),
            Line::from(vec![Span::styled(
                "     let theme = theme_data::tokyo_night();",
                base,
            )]),
            Line::from(vec![Span::styled(
                "-    loop {",
                Style::default().fg(c(1)).bg(bg),
            )]),
            Line::from(vec![Span::styled(
                "+    let mut view = View::Swatches;",
                Style::default().fg(c(2)).bg(bg),
            )]),
            Line::from(vec![Span::styled(
                "+",
                Style::default().fg(c(2)).bg(bg),
            )]),
            Line::from(vec![Span::styled(
                "+    loop {",
                Style::default().fg(c(2)).bg(bg),
            )]),
            Line::from(vec![Span::styled(
                "         terminal.draw(|frame| {",
                base,
            )]),
            Line::from(""),
            // --- Section 3: shell prompt + ls -la ---
            {
                let mut spans = prompt_spans();
                spans.push(Span::styled("ls -la", base));
                Line::from(spans)
            },
            Line::from(vec![
                Span::styled("total 48", base),
            ]),
            Line::from(vec![
                Span::styled("drwxr-xr-x  5 user user 4096 Mar 20 09:15 ", base),
                Span::styled(".", Style::default().fg(c(12)).bg(bg)),
            ]),
            Line::from(vec![
                Span::styled("drwxr-xr-x 12 user user 4096 Mar 19 14:22 ", base),
                Span::styled("..", Style::default().fg(c(12)).bg(bg)),
            ]),
            Line::from(vec![
                Span::styled("-rw-r--r--  1 user user  284 Mar 20 08:40 ", base),
                Span::styled(".gitignore", Style::default().fg(c(8)).bg(bg)),
            ]),
            Line::from(vec![
                Span::styled("-rw-r--r--  1 user user 1024 Mar 20 09:10 ", base),
                Span::styled("Cargo.toml", base),
            ]),
            Line::from(vec![
                Span::styled("drwxr-xr-x  3 user user 4096 Mar 20 09:15 ", base),
                Span::styled("src", Style::default().fg(c(12)).bg(bg)),
            ]),
            Line::from(vec![
                Span::styled("-rwxr-xr-x  1 user user 8192 Mar 20 09:15 ", base),
                Span::styled("target/debug/myapp", Style::default().fg(c(10)).bg(bg)),
            ]),
            Line::from(vec![
                Span::styled("lrwxrwxrwx  1 user user   12 Mar 18 11:00 ", base),
                Span::styled("latest -> target/debug/myapp", Style::default().fg(c(14)).bg(bg)),
            ]),
        ];

        for (i, line) in lines.iter().enumerate() {
            let y = area.top() + i as u16;
            if y >= area.bottom() {
                break;
            }
            let rect = Rect {
                x: area.left(),
                y,
                width: area.width,
                height: 1,
            };
            line.clone().render(rect, buf);
        }
    }
}
