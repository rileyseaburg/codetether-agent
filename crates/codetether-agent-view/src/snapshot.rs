//! Common data projected for local children and observed peers.
use crate::AgentOrigin;

/// A read-only local-child or peer snapshot, suitable for UI display.
/// # Examples
/// Obtain snapshots with [crate::list] and distinguish origins with [AgentOrigin].
#[derive(Clone, Debug)]
pub struct AgentSnapshot {
    pub id: String,
    pub name: String,
    pub instructions: String,
    pub message_count: usize,
    pub depth: u8,
    pub is_processing: bool,
    /// Where the agent came from; determines which identity rows make sense.
    pub origin: AgentOrigin,
    /// Whether the latest observed turn failed.
    pub failed: bool,
}
