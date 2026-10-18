//! Background RLM cache for session-context compaction.

use std::sync::Arc;

use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;

use super::{cache, job, thread};

pub(super) fn summary_or_spawn(
    content: &str,
    reason: &str,
    session_id: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
    resolver: crate::Resolver,
) -> Option<String> {
    let input = serde_json::json!({ "reason": reason });
    let key = cache::key("session_context", &input, content);
    if let Some(summary) = cache::ready(key) {
        return Some(summary);
    }
    if cache::claim(key) {
        thread::spawn(job::Job {
            key,
            content: content.into(),
            tool: "session_context".into(),
            input,
            session_id: session_id.into(),
            model: model.into(),
            provider,
            config: config.clone(),
            reason: reason.into(),
            original_bytes: content.len(),
            notify: None,
            resolver,
        });
    }
    None
}
