use super::FrameId;
use crate::browser_agent::Origin;

/// Metadata for one deterministic frame `postMessage` attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameMessage {
    /// Monotonic sequence number assigned by the page queue.
    pub sequence: u64,
    /// Source frame that posted the message.
    pub source: FrameId,
    /// Target frame that received or rejected the message.
    pub target: FrameId,
    /// String payload carried by this bounded message model.
    pub data: String,
    /// Caller-provided `targetOrigin` filter.
    pub target_origin_filter: String,
    /// Origin metadata for the source frame.
    pub source_origin: Origin,
    /// Origin metadata for the target frame.
    pub target_origin: Origin,
    /// Whether source and target origins match.
    pub same_origin: bool,
    /// Whether the page security policy allows the target origin.
    pub allowed_by_policy: bool,
    /// Whether this message passed target-origin and policy checks.
    pub delivered: bool,
}
