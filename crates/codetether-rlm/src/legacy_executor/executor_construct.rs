//! Construct the shared legacy executor from explicit host settings.
use super::*;
impl RlmExecutor {
    /// Create a new RLM executor
    ///
    /// # Arguments
    /// * `context` - Text loaded into the in-memory REPL.
    /// * `provider` - Shared model provider used by analysis and sub-queries.
    /// * `model` - Provider-local model identifier.
    /// * `context_budget_tokens` - Token budget for the context trace.
    /// * `tool_router` - Optional host response-rewriting service.
    /// # Returns
    /// A legacy executor using the caller's context budget and rewriter.
    /// # Examples
    /// See the construction example on [`RlmExecutor`].
    pub fn new(
        context: String,
        provider: Arc<dyn Provider>,
        model: String,
        context_budget_tokens: usize,
        tool_router: Option<Arc<dyn ResponseRewriter>>,
    ) -> Self {
        Self {
            repl: RlmRepl::new(context, ReplRuntime::Rust),
            provider,
            model,
            analysis_temperature: 0.3,
            max_iterations: 5, // Keep iterations limited for speed
            sub_queries: Vec::new(),
            trace_steps: Vec::new(),
            context_budget_tokens,
            context_trace: ContextTrace::new(context_budget_tokens),
            verbose: false,

            tool_router,
        }
    }
}
