//! Hybrid tool-call router backed by a local FunctionGemma model.

use std::sync::{Arc, Mutex};

use crate::CandleThinker as CandleRuntime;

/// Router that rewrites text-only responses into structured tool calls.
///
/// Created once at session start; shared via `Arc` across prompt calls.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_native::routing::{ToolCallRouter, ToolRouterConfig};
/// let router = ToolCallRouter::from_config(&ToolRouterConfig::default()).unwrap();
/// assert!(router.is_none());
/// ```
pub struct ToolCallRouter {
    pub(super) runtime: Arc<Mutex<CandleRuntime>>,
}

impl std::fmt::Debug for ToolCallRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolCallRouter").finish()
    }
}
