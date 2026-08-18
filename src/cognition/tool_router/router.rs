//! Hybrid tool-call router backed by a local FunctionGemma model.

use std::sync::{Arc, Mutex};

use crate::cognition::thinker::CandleRuntime;

/// Router that rewrites text-only responses into structured tool calls.
///
/// Created once at session start; shared via `Arc` across prompt calls.
pub struct ToolCallRouter {
    pub(super) runtime: Arc<Mutex<CandleRuntime>>,
}

impl std::fmt::Debug for ToolCallRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolCallRouter").finish()
    }
}
