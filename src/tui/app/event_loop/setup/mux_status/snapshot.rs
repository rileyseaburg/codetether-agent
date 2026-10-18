//! Borrow native session and UI fields for shared mux report policy.
use crate::tui::app::{session_runtime::SessionView, state::App};
use codetether_mux_model::RuntimeView;
pub(super) fn view<'a>(app: &'a App, session: &'a SessionView) -> RuntimeView<'a> {
    RuntimeView {
        session_id: &session.id,
        title: session.title.as_deref(),
        processing: app.state.processing,
        message_count: session.message_count,
        current_tool: app.state.tool_timing.pending_tool_name.as_deref(),
        input: &app.state.input,
        watchdog_pending: app.state.watchdog_notification.is_some(),
        restarts: app.state.main_watchdog_restart_count,
        status: &app.state.status,
        principal: &session.principal,
    }
}
#[cfg(test)]
fn unchanged(last: &crate::mux::MuxRuntimeStatus, app: &App, session: &SessionView) -> bool {
    view(app, session).unchanged(last)
}
#[cfg(test)]
fn build(app: &App, session: &SessionView) -> crate::mux::MuxRuntimeStatus {
    view(app, session).snapshot()
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
