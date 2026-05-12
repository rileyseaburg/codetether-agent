use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Summary statistics for a context trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTraceSummary {
    /// Total tokens used.
    pub total_tokens: usize,
    /// Maximum token budget.
    pub max_tokens: usize,
    /// Percentage of budget used.
    pub budget_used_percent: f32,
    /// Current iteration number.
    pub iteration: usize,
    /// Count of events by type.
    pub event_counts: HashMap<String, usize>,
    /// Tokens by event type.
    pub event_tokens: HashMap<String, usize>,
    /// Total number of events.
    pub events_len: usize,
}
