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
