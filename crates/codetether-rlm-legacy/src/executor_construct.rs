//! Initialize host settings before constructing the shared executor.
use super::*;
impl RlmExecutor {
    /// Create a new RLM executor
    ///
    /// # Arguments
    /// * `context` - Text loaded into the in-memory REPL.
    /// * `provider` - Shared model provider used by analysis and sub-queries.
    /// * `model` - Provider-local model identifier.
    /// # Returns
    /// A legacy executor with environment-derived context and router settings.
    /// # Examples
    /// See the construction example on [`RlmExecutor`].
    pub fn new(context: String, provider: Arc<dyn Provider>, model: String) -> Self {
        let context_budget_tokens = std::env::var("CODETETHER_RLM_CONTEXT_BUDGET")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(32_768);

        let tool_router = {
            let cfg = ToolRouterConfig::from_env();
            ToolCallRouter::from_config(&cfg)
                .inspect_err(|e| {
                    tracing::debug!(target: "codetether_agent::rlm::repl::executor_construct",error = %e, "FunctionGemma router unavailable for RLM");
                })
                .ok()
                .flatten()
        };

        Self {
            inner: codetether_rlm::legacy_executor::RlmExecutor::new(
                context,
                provider,
                model,
                context_budget_tokens,
                tool_router
                    .map(|router| Arc::new(RouterRewriter(router)) as Arc<dyn ResponseRewriter>),
            ),
        }
    }
}
