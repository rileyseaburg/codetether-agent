//! Guarded TUI drawing.
//!
//! This module provides a small safety wrapper around ratatui rendering. Some
//! terminal backends can briefly report invalid or extremely large dimensions,
//! especially during resize events or when running under unusual terminal
//! emulators. Rendering with those dimensions can allocate excessive buffers or
//! otherwise make the UI unresponsive.
//!
//! The public entry point, [`draw_ui`], checks the current terminal size before
//! delegating to the main UI renderer. Invalid sizes are logged and skipped
//! rather than treated as fatal application errors.

use ratatui::{Terminal, backend::CrosstermBackend};

#[path = "safe_draw_size.rs"]
mod safe_draw_size;

use crate::tui::{app::session_runtime::SessionView, app::state::App, ui::main::ui};
use safe_draw_size::safe_size;

/// Draw one TUI frame when the reported terminal size is sane.
///
/// The function reads the terminal's current size, rejects zero-sized or
/// pathologically large dimensions, and then delegates to the main [`ui`]
/// renderer. When the size is unsafe, it logs a warning and returns `Ok(())` so
/// the event loop can continue and try again on the next tick.
///
/// # Arguments
///
/// * `terminal` - The ratatui terminal backed by Crossterm stdout.
/// * `app` - Mutable application state used by the UI renderer.
/// * `session` - Read-only session view rendered in the conversation panel.
///
/// # Returns
///
/// Returns `Ok(())` when drawing succeeds or when drawing is intentionally
/// skipped because the terminal size is unsafe.
///
/// # Errors
///
/// Returns an error if querying the terminal size fails or if ratatui fails to
/// render the frame.
///
/// # Side Effects
///
/// Writes a frame to the terminal when rendering proceeds. Emits a tracing
/// warning when rendering is skipped due to invalid dimensions.
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &SessionView,
) -> anyhow::Result<()> {
    record_context(app, session);
    let size = terminal.size()?;
    if !safe_size(size) {
        log_skipped_size(size);
        return Ok(());
    }
    terminal.draw(|f| ui(f, app, session))?;
    Ok(())
}

fn record_context(app: &App, session: &SessionView) {
    crate::telemetry::crash_context::record_tui(
        &session.id,
        session.message_count,
        session.model.as_deref(),
        session.directory.as_deref(),
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
