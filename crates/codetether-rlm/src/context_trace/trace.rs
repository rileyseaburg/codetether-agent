use super::ContextEvent;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Maximum number of events to keep in the trace buffer.
pub(super) const MAX_EVENTS: usize = 1000;

/// Context trace for a single RLM analysis run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTrace {
    /// Maximum token budget.
    pub(super) max_tokens: usize,
    /// Events logged during the run.
    pub(super) events: VecDeque<ContextEvent>,
    /// Total tokens accumulated.
    pub(super) total_tokens: usize,
    /// Iteration number.
    pub(super) iteration: usize,
}

impl ContextTrace {
    /// Create a new context trace with the given token budget.
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            events: VecDeque::with_capacity(64),
            total_tokens: 0,
            iteration: 0,
        }
    }
}
