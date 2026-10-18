//! Native session binding for shared held-answer delivery.
use crate::provider::ProviderRegistry;
use crate::tui::app::{session_runtime::SessionSlot, state::App};
use std::sync::Arc;
pub(crate) async fn deliver(
    app: &mut App,
    slot: &SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
) -> bool {
    codetether_goal_answer::deliver(&mut super::delivery_host::Native {
        app,
        slot,
        registry,
    })
    .await
}
#[cfg(test)]
#[path = "answer_tests.rs"]
mod tests;
