//! Native grant lookup layered on the common MCP policy evaluator.
use crate::inbound::Evaluator;
use serde_json::Value;

/// Check existing alias approvals using the application's runtime policy.
///
/// # Implementors
/// Implement the existing evaluator and provide session-aware grant lookup.
pub trait Host: Evaluator {
    /// Report whether this exact invocation has an approved grant.
    fn approved(name: &str, args: &Value) -> bool;
}
