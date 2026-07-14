//! Guarded TUI drawing.
//!
//! This module provides a multi-layer safety wrapper around ratatui rendering:
//!
//! 1. **Size guard** — rejects zero-sized or pathologically large dimensions
//!    reported during resize events or under unusual terminal emulators.
//! 2. **Panic guard** — catches any panic inside the render closure so a
//!    single bad frame never kills the process (critical for long-running
//!    sessions that accumulate diverse message content).
//! 3. **Error recovery** — interrupted draws clear the retained terminal state
//!    and retry once so the physical screen and ratatui buffers stay aligned.
//!
//! The public entry point, [`draw_ui`], always returns `Ok(())` unless the
//! terminal backend is fundamentally broken, in which case the caller can
//! still continue the loop.

use ratatui::{Terminal, backend::CrosstermBackend};

#[path = "safe_draw_recover.rs"]
mod safe_draw_recover;
#[path = "safe_draw_resync.rs"]
mod safe_draw_resync;
#[path = "safe_draw_size.rs"]
mod safe_draw_size;

use crate::tui::{app::session_runtime::SessionView, app::state::App};
use safe_draw_recover::draw_or_recover;
use safe_draw_size::safe_size;

/// Draw one TUI frame, recovering from transient failures.
///
/// The function reads the terminal size, rejects unsafe dimensions, and
/// delegates to the panic-guarded renderer. Size-query errors, draw errors,
/// and render-closure panics are all logged and swallowed so the event loop
/// continues after clearing and retrying the interrupted frame once.
///
/// # Side Effects
///
/// Writes a frame to stdout on success. Emits `tracing` warnings/errors on
/// any recoverable failure.
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &SessionView,
) -> anyhow::Result<()> {
    record_context(app, session);
    let size = match terminal.size() {
        Ok(s) => s,
        Err(err) => {
            tracing::warn!(%err, "terminal size query failed — skipping frame");
            return Ok(());
        }
    };
    if !safe_size(size) {
        log_skipped_size(size);
        return Ok(());
    }
    if let Err(err) = draw_or_recover(terminal, app, session) {
        safe_draw_resync::clear_and_retry(terminal, app, session, err);
    }
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
