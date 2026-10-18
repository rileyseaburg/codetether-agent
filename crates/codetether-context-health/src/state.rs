//! Last observed context-health summaries.

/// Most recent human-readable summaries of context-management activity.
///
/// # Examples
/// ```
/// let state = codetether_context_health::ContextHealthState::default();
/// assert!(state.last_compaction.is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ContextHealthState {
    /// Latest completed compaction.
    pub last_compaction: Option<String>,
    /// Latest RLM progress or completion.
    pub last_rlm: Option<String>,
    /// Latest forced context truncation.
    pub last_truncation: Option<String>,
}
