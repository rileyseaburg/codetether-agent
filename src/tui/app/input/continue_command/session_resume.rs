//! Reuse native continuation operations for automatic resume and picker loads.
use super::*;
pub(crate) async fn resume_goal(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) -> bool {
    codetether_chat_submit::resume::automatic::run(
        &mut host::Native {
            app,
            cwd,
            slot,
            registry,
            worker_bridge,
            runtime,
        },
        registry.is_some(),
    )
    .await
}
pub(in crate::tui::app::input) async fn select_session(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    codetether_session_picker::interaction::select(&mut host::Native {
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        runtime,
    })
    .await;
}
