use tokio::sync::mpsc;
use std::time::Instant;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use crate::tui::app::state::{App, SessionEvent};
use crate::tui::ui::main::ui;

pub async fn run(_project: Option<std::path::PathBuf>) -> anyhow::Result<()> {
    // Basic terminal setup stub
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::default();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // Basic exit for now
        break;
    }
    Ok(())
}
