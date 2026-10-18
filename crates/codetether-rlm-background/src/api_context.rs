//! Public context_summary entry point.
use crate::Resolver;
use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;
use std::sync::Arc;

/// Return a cached context summary or start a background job.
///
/// The resolver runs inside the worker and retains application model policy.
/// Cache keys and completed results are shared across calls in the process.
/// # Arguments
/// * `content` — Context text to summarize.
/// * `reason` — Compression trigger used in the cache key.
/// * `session_id` — Session identifier for job traces.
/// * `model` — Caller model selector.
/// * `provider` — Caller provider.
/// * `config` — RLM configuration copied into a new job.
/// * `resolver` — Native model-resolution callback.
/// # Returns
/// A ready cached summary, or None after claiming/scheduling work.
pub fn context_summary(
    content: &str,
    reason: &str,
    session_id: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
    resolver: Resolver,
) -> Option<String> {
    crate::context::summary_or_spawn(
        content, reason, session_id, model, provider, config, resolver,
    )
}
