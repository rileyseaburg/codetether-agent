//! Routing facade over the shared RLM decision and processing engine.
use crate::{AutoProcessContext, RoutingContext, RoutingResult};
use anyhow::Result;
use codetether_rlm::RlmConfig;
use codetether_rlm::RlmResult;
use codetether_rlm::router;

/// RLM Router — delegates to [`codetether_rlm::router`].
/// # Examples
/// ```
/// let _router = codetether_rlm_routing::RlmRouter;
/// ```
pub struct RlmRouter;

impl RlmRouter {
    /// Check if tool output should be routed through RLM.
    /// # Arguments
    /// Receives the tool output, routing context and current RLM configuration.
    /// # Returns
    /// The shared routing decision and reason.
    /// # Examples
    /// ```rust,no_run
    /// # fn example(output: &str, context: &codetether_rlm_routing::RoutingContext, config: &codetether_rlm::RlmConfig) {
    /// let _decision = codetether_rlm_routing::RlmRouter::should_route(output, context, config);
    /// # }
    /// ```
    pub fn should_route(output: &str, ctx: &RoutingContext, config: &RlmConfig) -> RoutingResult {
        router::should_route(output, ctx, config)
    }

    /// Smart truncate large output with RLM hint.
    /// # Arguments
    /// Receives output, originating tool/input and the truncation limit.
    /// # Returns
    /// The shared truncated text, truncation flag and size accounting.
    /// # Examples
    /// ```
    /// let (text, truncated, _) = codetether_rlm_routing::RlmRouter::smart_truncate("short", "read", &serde_json::json!({}), 100);
    /// assert_eq!(text, "short");
    /// assert!(!truncated);
    /// ```
    pub fn smart_truncate(
        output: &str,
        tool_id: &str,
        args: &serde_json::Value,
        max: usize,
    ) -> (String, bool, usize) {
        router::smart_truncate(output, tool_id, args, max)
    }

    /// Automatically process large output through RLM.
    /// # Arguments
    /// Receives output, the owned processing context and RLM configuration.
    /// # Returns
    /// The shared RLM processing result.
    /// # Errors
    /// Propagates provider, REPL and processing errors.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example<B: codetether_rlm_routing::Bus>(ctx: codetether_rlm_routing::AutoProcessContext<'_, B>, config: &codetether_rlm::RlmConfig) -> anyhow::Result<()> {
    /// let _result = codetether_rlm_routing::RlmRouter::auto_process("output", ctx, config).await?;
    /// # Ok(()) }
    /// ```
    pub async fn auto_process<B: crate::Bus>(
        output: &str,
        ctx: AutoProcessContext<'_, B>,
        config: &RlmConfig,
    ) -> Result<RlmResult> {
        let crate_ctx = crate::convert_ctx(ctx);
        router::auto_process_with_repl(output, crate_ctx, config).await
    }
}
