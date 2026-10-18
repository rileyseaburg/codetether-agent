//! Shared execution contracts and live application tool dispatch.
use super::ThoughtEvent;
use super::thinker::ThinkerClient;
use crate::tool::ToolRegistry;
pub use codetether_cognition_core::execution::{
    CapabilityLease, DecisionReceipt, ExecutionOutcome, ToolInvocation,
};
use std::sync::Arc;
#[path = "executor/registry.rs"]
mod registry;
/// Execute thought-extracted requests through the shared cognition dispatcher.
pub async fn execute_tool_requests(
    thinker: Option<&ThinkerClient>,
    tool_registry: &Arc<ToolRegistry>,
    persona_id: &str,
    thought_text: &str,
    allowed_tools: &[String],
) -> Vec<ThoughtEvent> {
    codetether_cognition_core::execution::dispatch::execute_tool_requests(
        thinker,
        tool_registry,
        persona_id,
        thought_text,
        allowed_tools,
    )
    .await
}
#[cfg(test)]
#[path = "executor/tests.rs"]
mod tests;
