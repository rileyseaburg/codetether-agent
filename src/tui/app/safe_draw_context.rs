//! Crash context and diagnostics recorded around guarded TUI draws.

use crate::tui::app::{session_runtime::SessionView, state::App};

pub(super) fn record(app: &App, session: &SessionView) {
    crate::telemetry::crash_context::record_tui(
        &session.id,
        session.message_count,
        session.model.as_deref(),
        session.directory.as_deref(),
        &app.state.status,
    );
}

pub(super) fn log_skipped_size(size: ratatui::layout::Size) {
    tracing::warn!(
        width = size.width,
        height = size.height,
        cells = size.width as u32 * size.height as u32,
        "skipping TUI draw because terminal reported an invalid size"
    );
}
