//! Run foreground RLM compaction and recover with deterministic compression.
use codetether_provider_core::Provider;
use codetether_rlm::router::AutoProcessContext;
use codetether_rlm::{RlmChunker, RlmConfig, RlmRouter};
use std::sync::Arc;

/// Summarize context using a same-provider model and deterministic fallback.
/// # Arguments
/// * `context` — Text requiring compaction.
/// * `ctx_window` — Model context window used to size the fallback.
/// * `session_id` — Owning session identifier for RLM traces.
/// * `reason` — Compaction trigger included in router arguments.
/// * `model` — Caller model used when alternatives are unavailable.
/// * `provider` — Provider supplying both model discovery and execution.
/// * `config` — RLM router settings.
/// # Returns
/// None for a zero-token input; otherwise an RLM or deterministic summary.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let config = codetether_rlm::RlmConfig::default();
/// let result = codetether_rlm_foreground::context_summary(
///     "", 128_000, "session", "budget", "model", provider, &config).await;
/// assert!(result.is_none());
/// # }
/// ```
pub async fn context_summary(
    context: &str,
    ctx_window: usize,
    session_id: &str,
    reason: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
) -> Option<String> {
    if RlmChunker::estimate_tokens(context) == 0 {
        return None;
    }
    let (provider, model) = super::model::resolve(provider, model).await;
    let request = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({ "reason": reason }),
        session_id,
        abort: None,
        on_progress: None,
        provider,
        model,
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
    };
    match RlmRouter::auto_process(context, request, config).await {
        Ok(result) => Some(result.processed),
        Err(error) => {
            tracing::warn!(target: "codetether_agent::session::helper::compression::compression_defer", %error, "Foreground RLM compaction failed; using deterministic compression");
            Some(RlmChunker::compress(context, ctx_window / 4, None))
        }
    }
}
