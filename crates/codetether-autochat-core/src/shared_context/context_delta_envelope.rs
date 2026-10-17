//! Shared relay context data.
use super::*;

/// Relay and turn provenance for one structured context delta.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::ContextDeltaEnvelope;
/// let envelope = ContextDeltaEnvelope { relay_id: "demo".into(), from_agent: "worker".into(), round: 0, turn: 0, delta: Default::default() };
/// assert_eq!(envelope.turn, 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDeltaEnvelope {
    pub relay_id: String,
    pub from_agent: String,
    pub round: usize,
    pub turn: usize,
    pub delta: ContextDelta,
}
