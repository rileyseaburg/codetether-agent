//! Native UI projection for shared mux status reporting.
mod snapshot;
use crate::tui::app::{session_runtime::SessionView, state::App};
#[derive(Default)]
pub(in crate::tui::app::event_loop) struct Reporter {
    inner: codetether_mux_control::RuntimeReporter,
}
impl Reporter {
    pub(in crate::tui::app::event_loop) async fn update(
        &mut self,
        app: &App,
        session: &SessionView,
    ) {
        self.inner.update(snapshot::view(app, session)).await;
    }
    pub(in crate::tui::app::event_loop) async fn clear(self) {
        self.inner.clear().await;
    }
}
