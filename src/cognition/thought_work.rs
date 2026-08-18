//! Internal work item and result types for one thought tick.

use super::ThoughtPhase;

/// One persona's pending thought for the current tick.
#[derive(Debug, Clone)]
pub(super) struct ThoughtWorkItem {
    pub persona_id: String,
    pub persona_name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub thought_count: u64,
    pub phase: ThoughtPhase,
}

/// Outcome of generating one thought, including provenance and cost.
#[derive(Debug, Clone)]
pub(super) struct ThoughtResult {
    pub source: &'static str,
    pub model: Option<String>,
    pub finish_reason: Option<String>,
    pub thinking: String,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub latency_ms: u128,
    pub error: Option<String>,
}
