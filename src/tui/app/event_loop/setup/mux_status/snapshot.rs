//! Borrowed comparison and owned construction for mux runtime status.

use crate::mux::MuxRuntimeStatus;
use crate::tui::app::{session_runtime::SessionView, state::App};

pub(super) fn unchanged(last: &MuxRuntimeStatus, app: &App, session: &SessionView) -> bool {
    last.session_id == session.id
        && last.session_title == session.title.as_deref().unwrap_or("Untitled session")
        && last.processing == app.state.processing
        && last.message_count == session.message_count
        && last.current_tool.as_deref() == app.state.pending_tool_name.as_deref()
        && last.needs_interaction == !app.state.input.trim().is_empty()
        && last.lagging == lagging(app)
        && last.principal == session.principal
}

pub(super) fn build(app: &App, session: &SessionView) -> MuxRuntimeStatus {
    MuxRuntimeStatus {
        session_id: session.id.clone(),
        session_title: session
            .title
            .clone()
            .unwrap_or_else(|| "Untitled session".into()),
        processing: app.state.processing,
        message_count: session.message_count,
        current_tool: app.state.pending_tool_name.clone(),
        needs_interaction: !app.state.input.trim().is_empty(),
        lagging: lagging(app),
        principal: session.principal.clone(),
    }
}

fn lagging(app: &App) -> bool {
    app.state.watchdog_notification.is_some()
        || (app.state.processing && app.state.main_watchdog_restart_count > 0)
        || app.state.status.starts_with("Watchdog gave up")
}

#[cfg(test)]
mod tests {
    use super::{build, unchanged};
    use crate::tui::app::{session_runtime::SessionView, state::App};

    #[test]
    fn owned_status_matches_its_borrowed_source() {
        let app = App::default();
        let session = SessionView::default();
        assert!(unchanged(&build(&app, &session), &app, &session));
    }
}
