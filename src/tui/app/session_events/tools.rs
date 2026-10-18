//! Native bindings for shared tool-event presentation.
#[path = "approval.rs"]
mod approval;
mod timing;
mod view;
use crate::{
    session::SessionEvent,
    tui::{app::state::App, worker_bridge::TuiWorkerBridge},
};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
}
pub(super) async fn handle_event(
    app: &mut App,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    codetether_chat_events::tools::handle_event(&mut Native { app, worker_bridge }, evt).await
}
