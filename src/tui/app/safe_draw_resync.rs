//! Terminal-buffer resynchronization after an interrupted render pass.

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::tui::app::{session_runtime::SessionView, state::App};

/// Clear ratatui's retained screen state and retry one interrupted frame.
///
/// The initial error is logged. A failed clear or retry is also logged, but is
/// not propagated because the surrounding event loop must remain responsive.
pub(super) fn clear_and_retry(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &SessionView,
    initial_error: anyhow::Error,
) {
    tracing::warn!(error = %initial_error, "terminal draw interrupted; resynchronizing");
    if let Err(error) = terminal.clear() {
        tracing::error!(%error, "terminal resynchronization clear failed");
        return;
    }
    if let Err(error) = super::safe_draw_recover::draw_or_recover(terminal, app, session) {
        tracing::error!(%error, "terminal redraw after resynchronization failed");
    }
}
