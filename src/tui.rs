use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use std::io::stdout;
use std::io::Result;

use crate::smt_app::App;
use crate::view_ui;

use crossterm::event::{self, KeyCode, KeyEventKind};

/// TUI - terminal user interface
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn enter(&self) -> Result<()> {
        crossterm::execute!(stdout(), EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn exit(&self) -> Result<()> {
        crossterm::execute!(stdout(), LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| view_ui::render(app, frame))?;
        Ok(())
    }

    pub async fn handle_event(&mut self, app: &mut App) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    app.is_running = false;
                }

                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('r') {
                    app.data.match_information = String::from("Refreshing...");
                    app.data.table_1_league = String::from("Refreshing...");
                    app.data.table_super_league = String::from("Refreshing...");
                    let _ = self.draw(app);
                    app.fetch_data().await;
                }
            }
        }

        Ok(())
    }
}
