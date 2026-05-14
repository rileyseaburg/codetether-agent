//! Ordered session-event adapter pipeline.

use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn run(
    app: &mut App,
    session: &mut Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    mut evt: SessionEvent,
) -> Option<SessionEvent> {
    evt = super::lifecycle::handle_event(app, session, worker_bridge, evt).await?;
    evt = super::tools::handle_event(app, worker_bridge, evt).await?;
    evt = super::text_dispatch::handle_event(app, evt)?;
    evt = super::usage::handle_event(app, evt)?;
    super::errors::handle_event(app, session, worker_bridge, evt).await
}
