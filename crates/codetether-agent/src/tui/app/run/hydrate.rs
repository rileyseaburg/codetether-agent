//! Native completed-startup hydration entry.
use super::session_outcome::SessionLoadOutcome;
use crate::{
    session::Session,
    tui::{app::state::App, models::WorkspaceSnapshot, worker_bridge::TuiWorkerBridge},
};
pub(super) fn complete(
    app: &mut App,
    allow_network: bool,
    session: &mut Session,
    worker_bridge: Option<&TuiWorkerBridge>,
    outcome: SessionLoadOutcome,
    workspace: WorkspaceSnapshot,
) {
    codetether_tui_startup::hydrate::complete(
        &mut super::hydrate_host::Native(app),
        allow_network,
        session,
        worker_bridge,
        outcome,
        workspace,
    );
}
