//! Announce the worker's real tool schemas on the agent bus.
//!
//! Recording the concrete schemas (not just capability names) lets exported
//! training transcripts reproduce the inference-time tools prompt, so a model
//! fine-tuned on CodeTether traffic learns to emit tool calls rather than
//! echoing the tool list back.

use crate::bus::BusHandle;
use crate::bus::tool_catalog::from_definitions;
use crate::tool::ToolRegistry;

/// Publish the default worker tool catalog for training fidelity.
pub fn announce(handle: &BusHandle) -> usize {
    let schemas = from_definitions(&ToolRegistry::new().definitions());
    let count = schemas.len();
    let receivers = handle.announce_tool_catalog(schemas);
    tracing::info!(
        tools = count,
        "Announced worker tool catalog for training capture"
    );
    receivers
}
