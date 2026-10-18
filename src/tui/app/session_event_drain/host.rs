//! Native event handling for shared bounded delivery.
use super::{App, SessionEvent, SessionSlot, TuiWorkerBridge};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
}
impl codetether_session_events::drain::Handler<crate::session::Session> for Native<'_> {
    async fn handle(&mut self, event: SessionEvent) {
        crate::tui::app::session_events::handle_session_event(
            self.app,
            self.slot,
            self.worker_bridge,
            event,
        )
        .await;
    }
}
