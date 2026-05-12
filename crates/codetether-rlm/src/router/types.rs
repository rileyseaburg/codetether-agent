//! Public types used by all router submodules.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::traits::{LlmProvider, RlmEventBus, ToolCallRewriter};

/// Context for a routing decision.
#[derive(Debug, Clone)]
pub struct RoutingContext {
    pub tool_id: String,
    pub session_id: String,
    pub call_id: Option<String>,
    pub model_context_limit: usize,
    pub current_context_tokens: Option<usize>,
}

/// Outcome of a routing decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingResult {
    pub should_route: bool,
    pub reason: String,
    pub estimated_tokens: usize,
}

/// Progress tick during auto-processing.
#[derive(Debug, Clone)]
pub struct ProcessProgress {
    pub iteration: usize,
    pub max_iterations: usize,
    pub status: String,
}

/// Internal context for the crate's auto-process loop.
///
/// The host crate converts its own context into this via
/// [`IntoCrateCtx`]. End users never construct this directly.
pub struct CrateAutoProcessContext<'a> {
    pub tool_id: &'a str,
    pub tool_args: serde_json::Value,
    pub session_id: &'a str,
    pub abort: Option<tokio::sync::watch::Receiver<bool>>,
    pub on_progress: Option<Box<dyn Fn(ProcessProgress) + Send + Sync>>,
    pub provider: Arc<dyn LlmProvider>,
    pub model: String,
    pub bus: Option<Arc<dyn RlmEventBus>>,
    pub trace_id: Option<Uuid>,
    pub subcall_provider: Option<Arc<dyn LlmProvider>>,
    pub subcall_model: Option<String>,
    pub rewriter: Option<Arc<dyn ToolCallRewriter>>,
}

/// Trait for converting a host context into the crate's internal form.
pub trait IntoCrateCtx<'a> {
    /// Convert into the crate's internal context.
    fn into_crate_ctx(self) -> CrateAutoProcessContext<'a>;
}

/// Outcome of the iterative RLM loop.
pub struct LoopOutcome {
    pub final_answer: Option<String>,
    pub iterations: usize,
    pub subcalls: usize,
    pub aborted: bool,
    pub last_error: Option<String>,
}
