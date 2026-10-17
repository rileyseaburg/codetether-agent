//! State owned by the shared legacy RLM executor.
use super::*;

/// LLM-powered RLM executor
///
/// This is the main entry point for RLM processing. It:
/// 1. Loads context into a REPL environment
/// 2. Lets the LLM write analysis code
/// 3. Executes the code and provides llm_query() for semantic sub-calls
/// 4. Iterates until the LLM returns a FINAL answer
///
/// The executor passes tool definitions to the provider and dispatches structured
/// calls when present, falling back to DSL code blocks. An optional host
/// [`ResponseRewriter`] runs before either dispatch path.
///
/// # Examples
/// ```no_run
/// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let executor = codetether_rlm::legacy_executor::RlmExecutor::new(
///     "context".into(), provider, "model".into(), 32_768, None,
/// );
/// assert!(executor.trace_steps().is_empty());
/// # }
/// ```
pub struct RlmExecutor {
    pub(super) repl: RlmRepl,
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) analysis_temperature: f32,
    pub(super) max_iterations: usize,
    pub(super) sub_queries: Vec<SubQuery>,
    pub(super) trace_steps: Vec<TraceStep>,
    pub(super) context_budget_tokens: usize,
    pub(super) context_trace: ContextTrace,
    pub(super) verbose: bool,

    pub(super) tool_router: Option<Arc<dyn ResponseRewriter>>,
}
