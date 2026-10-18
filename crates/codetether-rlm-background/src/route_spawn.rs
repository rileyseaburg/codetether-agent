//! Claim and launch a background job for one routed tool result.
use super::{Notify, Resolver, cache, job, thread};
use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;
use std::sync::Arc;
pub(super) fn start(
    key: u64,
    rendered: &str,
    tool: &str,
    input: &serde_json::Value,
    session_id: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
    notify: Option<Notify>,
    reason: &str,
    resolver: Resolver,
) {
    if cache::claim(key) {
        thread::spawn(job::Job {
            key,
            content: rendered.into(),
            tool: tool.into(),
            input: input.clone(),
            session_id: session_id.into(),
            model: model.into(),
            provider,
            config: config.clone(),
            reason: reason.to_string(),
            original_bytes: rendered.len(),
            notify,
            resolver,
        });
    }
}
