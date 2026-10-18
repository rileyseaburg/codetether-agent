//! Bind native lifecycle operations to shared session-event transitions.
use super::host::Native;
use crate::{
    session::SessionEvent,
    tui::{
        app::{session_runtime::SessionSlot, state::App},
        worker_bridge::TuiWorkerBridge,
    },
};
mod completion;
mod processing;
mod sync;
pub(super) async fn handle_event(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) -> Option<SessionEvent> {
    codetether_session_events::lifecycle::handle_event(
        &mut Native {
            app,
            slot,
            worker_bridge,
        },
        evt,
    )
    .await
}
