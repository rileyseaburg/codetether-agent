//! Ordered session-event adapter pipeline.

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn run(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    mut evt: SessionEvent,
) -> Option<SessionEvent> {
    evt = super::lifecycle::handle_event(app, slot, worker_bridge, evt).await?;
    evt = super::interlude::handle_event(app, evt)?;
    evt = super::retry::handle_event(app, evt)?;
    evt = super::tools::handle_event(app, worker_bridge, evt).await?;
    evt = super::text_dispatch::handle_event(app, evt)?;
    evt = super::usage::handle_event(app, evt)?;
    super::errors::handle_event(app, slot, worker_bridge, evt).await
}
