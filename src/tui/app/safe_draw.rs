//! Guarded TUI drawing.

use ratatui::{Terminal, backend::CrosstermBackend};

#[path = "safe_draw_size.rs"]
mod safe_draw_size;

use crate::session::Session;
use crate::tui::{app::state::App, ui::main::ui};
use safe_draw_size::{rect_for, safe_size};

/// Draw one TUI frame when the reported terminal size is sane.
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &Session,
) -> anyhow::Result<()> {
    record_context(app, session);
    let size = terminal.size()?;
    if !safe_size(size) {
        log_skipped_size(size);
        return Ok(());
    }
    terminal.resize(rect_for(size))?;
    terminal.draw(|f| ui(f, app, session))?;
    Ok(())
}

fn record_context(app: &App, session: &Session) {
    crate::telemetry::crash_context::record_tui(
        &session.id,
        session.messages.len(),
        session.metadata.model.as_deref(),
        session.metadata.directory.as_deref(),
        &app.state.status,
    );
}

fn log_skipped_size(size: ratatui::layout::Size) {
    tracing::warn!(
        width = size.width,
        height = size.height,
        cells = size.width as u32 * size.height as u32,
        "skipping TUI draw because terminal reported an invalid size"
    );
}
