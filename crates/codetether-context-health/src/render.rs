//! Joined context usage and recent event output.
use crate::ContextHealthState;
use crate::lines::{compaction_line, rlm_line, truncation_line, usage_line};

/// Format current token usage and the most recent context-event summaries.
///
/// # Arguments
/// * `state` - Recorded context-management summaries.
/// * `used` - Current context token estimate.
/// * `budget` - Model context budget when known.
///
/// # Returns
/// Four labelled lines describing context usage and recent events.
///
/// # Examples
/// ```
/// let state = codetether_context_health::ContextHealthState::default();
/// assert!(codetether_context_health::render(&state, Some(50), Some(100)).contains("50.0%"));
/// ```
pub fn render(state: &ContextHealthState, used: Option<usize>, budget: Option<usize>) -> String {
    [
        usage_line(used, budget),
        compaction_line(state),
        rlm_line(state),
        truncation_line(state),
    ]
    .join("\n")
}
