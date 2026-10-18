//! Native session/runtime binding for shared watchdog retries.
mod host;
use crate::{
    provider::ProviderRegistry,
    tui::app::{
        session_runtime::{SessionSlot, TuiSessionHandle},
        state::App,
    },
};
use std::sync::Arc;
pub(super) async fn execute(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
) {
    codetether_watchdog::recovery::execute(
        &mut host::Native { app, slot, runtime },
        registry.as_ref(),
    )
    .await;
}
