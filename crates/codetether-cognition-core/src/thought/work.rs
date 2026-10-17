//! Internal work item and result types for one thought tick.

use super::ThoughtPhase;

/// One persona's pending thought for the current tick.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Observe };
/// assert_eq!(value.phase, ThoughtPhase::Observe);
/// ```
#[derive(Debug, Clone)]
pub struct ThoughtWorkItem {
    /// Persona associated with this record or request.
    pub persona_id: String,
    /// Human-readable name for this work item.
    pub persona_name: String,
    /// Role used for persona behavior and governance.
    pub role: String,
    /// Instructions defining the persona responsibility.
    pub charter: String,
    /// Optional owning swarm identifier.
    pub swarm_id: Option<String>,
    /// Number of thoughts produced by the persona.
    pub thought_count: u64,
    /// Phase to execute for this work item.
    pub phase: ThoughtPhase,
}

/// Outcome of generating one thought, including provenance and cost.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = ThoughtResult { source: "example", model: None, finish_reason: None, thinking: "Review complete".into(), prompt_tokens: None, completion_tokens: None, total_tokens: None, latency_ms: 0, error: None };
/// assert!(value.error.is_none());
/// ```
#[derive(Debug, Clone)]
pub struct ThoughtResult {
    /// Source of the generated thought.
    pub source: &'static str,
    /// Model used, if available.
    pub model: Option<String>,
    /// Provider completion reason, if available.
    pub finish_reason: Option<String>,
    /// Generated thought text.
    pub thinking: String,
    /// Reported input token usage.
    pub prompt_tokens: Option<u32>,
    /// Reported output token usage.
    pub completion_tokens: Option<u32>,
    /// Reported total token usage.
    pub total_tokens: Option<u32>,
    /// Elapsed generation time in milliseconds.
    pub latency_ms: u128,
    /// Generation failure text, if present.
    pub error: Option<String>,
}
