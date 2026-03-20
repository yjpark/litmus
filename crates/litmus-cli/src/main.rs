mod theme_data;
mod widgets;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use widgets::{MockupsWidget, SwatchesWidget};

enum View {
    Swatches,
    Mockups,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let theme = theme_data::tokyo_night();
    let mut view = View::Swatches;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            match view {
                View::Swatches => frame.render_widget(SwatchesWidget { theme: &theme }, area),
                View::Mockups => frame.render_widget(MockupsWidget { theme: &theme }, area),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab => {
                    view = match view {
                        View::Swatches => View::Mockups,
                        View::Mockups => View::Swatches,
                    };
                }
                _ => {}
            }
        }
    }
}
