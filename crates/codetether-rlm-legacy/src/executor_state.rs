//! State owned by the legacy RLM host executor.
use super::*;

/// LLM-powered RLM executor
///
/// This is the main entry point for RLM processing. It:
/// 1. Loads context into a REPL environment
/// 2. Lets the LLM write analysis code
/// 3. Executes the code and provides llm_query() for semantic sub-calls
/// 4. Iterates until the LLM returns a FINAL answer
///
/// When FunctionGemma is enabled, the executor passes RLM tool definitions to
/// the provider and dispatches structured tool calls instead of regex-parsing
/// DSL from code blocks.
///
/// # Examples
/// ```no_run
/// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let executor = codetether_rlm_legacy::RlmExecutor::new(
///     "context".into(), provider, "model".into(),
/// );
/// assert!(executor.trace_steps().is_empty());
/// # }
/// ```
pub struct RlmExecutor {
    pub(super) inner: codetether_rlm::legacy_executor::RlmExecutor,
}
