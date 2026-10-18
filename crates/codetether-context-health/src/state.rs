//! Last observed context usage and context-health summaries.

/// Latest token usage and human-readable context-management summaries.
///
/// # Examples
/// ```
/// let state = codetether_context_health::ContextHealthState::default();
/// assert!(state.last_compaction.is_none());
/// assert!(state.used_tokens.is_none());
/// assert!(state.token_budget.is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ContextHealthState {
    /// Latest reported context token estimate, when known.
    pub used_tokens: Option<usize>,
    /// Model context token budget, when known.
    pub token_budget: Option<usize>,
    /// Latest completed compaction.
    pub last_compaction: Option<String>,
    /// Latest RLM progress or completion.
    pub last_rlm: Option<String>,
    /// Latest forced context truncation.
    pub last_truncation: Option<String>,
}
