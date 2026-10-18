//! Bind deferred continuation to native session ownership.
use crate::provider::ProviderRegistry;
use crate::tui::{
    app::{
        session_runtime::{SessionSlot, TuiSessionHandle},
        state::App,
    },
    worker_bridge::TuiWorkerBridge,
};
use std::{path::Path, sync::Arc};
pub(crate) async fn drain(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    codetether_goal_command::input::drain(&mut super::resume_host::Native {
        app,
        cwd,
        slot,
        registry,
        bridge,
        runtime,
    })
    .await;
}
pub(crate) fn restart(app: &mut App, id: &str, runtime: &TuiSessionHandle) {
    codetether_goal_command::input::restart(&mut super::restart_host::Native { app, runtime }, id);
}
