//! Native lifecycle and failover bindings for error events.
#[path = "smart_switch.rs"]
mod smart_switch;
mod view;
use super::host::Native;
use crate::{
    session::SessionEvent,
    tui::{
        app::{session_runtime::SessionSlot, state::App},
        worker_bridge::TuiWorkerBridge,
    },
};
pub(super) async fn handle_event(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    codetether_chat_events::error::handle_event(
        &mut Native {
            app,
            slot,
            worker_bridge,
        },
        evt,
    )
    .await
}
