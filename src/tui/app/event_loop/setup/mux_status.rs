//! Change-driven semantic status reporting to an inherited mux server.

mod snapshot;

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
        if self
            .last
            .as_ref()
            .is_some_and(|last| snapshot::unchanged(last, app, session))
        {
            return;
        }
        let status = snapshot::build(app, session);
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
