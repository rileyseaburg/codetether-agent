//! Change-driven semantic status reporting to an inherited mux server.

use crate::mux::MuxRuntimeStatus;
use crate::tui::app::{session_runtime::SessionView, state::App};

#[derive(Default)]
pub(in crate::tui::app::event_loop) struct Reporter {
    last: Option<MuxRuntimeStatus>,
}

impl Reporter {
    pub(in crate::tui::app::event_loop) async fn update(
        &mut self,
        app: &App,
        session: &SessionView,
    ) {
        let status = MuxRuntimeStatus {
            session_id: session.id.clone(),
            session_title: session
                .title
                .clone()
                .unwrap_or_else(|| "Untitled session".into()),
            processing: app.state.processing,
            message_count: session.message_count,
            current_tool: app.state.pending_tool_name.clone(),
            needs_interaction: !app.state.input.trim().is_empty(),
            lagging: app.state.watchdog_notification.is_some()
                || (app.state.processing && app.state.main_watchdog_restart_count > 0)
                || app.state.status.starts_with("Watchdog gave up"),
        };
        if self.last.as_ref() == Some(&status) {
            return;
        }
        self.last = Some(status.clone());
        if let Err(error) = crate::mux::control::report_runtime(Some(status)).await {
            tracing::debug!(%error, "Mux runtime status unavailable");
        }
    }

    pub(in crate::tui::app::event_loop) async fn clear(self) {
        if self.last.is_some()
            && let Err(error) = crate::mux::control::report_runtime(None).await
        {
            tracing::debug!(%error, "Could not clear mux runtime status");
        }
    }
}
