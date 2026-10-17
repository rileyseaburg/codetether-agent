//! ToolCallRecord execution contract.
use super::*;

/// Record of a tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let call = codetether_swarm_core::ToolCallRecord { name: "read".into(), timestamp: chrono::Utc::now(), success: true };
/// assert!(call.success);
/// ```
pub struct ToolCallRecord {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
}
