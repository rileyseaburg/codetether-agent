//! Keep-last prefix compression: buffer core + owning-session wrapper.

use std::sync::Arc;

use anyhow::Result;

use codetether_context_policy::transcript::messages_to_rlm_context;
use codetether_provider_core::Message;
use codetether_provider_core::limits::context_window_for_model;

use super::context::CompressContext;

/// Compress everything older than the last `keep_last` messages in
/// `messages` into a single synthetic `[AUTO CONTEXT COMPRESSION]`
/// assistant message, in-place.
///
/// Callers provide a mutable projection or owned transcript; no native session
/// reference is required.
///
/// # Returns
///
/// `Ok(true)` if the buffer was rewritten, `Ok(false)` if it was already
/// short enough to skip compression.
///
/// # Errors
///
/// Propagates any error from the compression pipeline that cannot be
/// recovered by the deterministic chunker pass.
/// # Arguments
/// * `messages` — Mutable buffer or projection to compress.
/// * `ctx` — Owned session ID and RLM settings.
/// * `provider` — Caller provider used for summarization.
/// * `model` — Caller model used to derive context limits.
/// * `keep_last` — Requested recent-message count protected by active-tail selection.
/// * `reason` — Compression trigger passed to the RLM router.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, resolver: codetether_rlm_background::Resolver) {
/// let ctx = codetether_context_compression::CompressContext::new("session", &Default::default());
/// let mut messages = Vec::new();
/// let changed = codetether_context_compression::compress_messages_keep_last(&mut messages, &ctx, provider, "model", 0, "budget").await.unwrap();
/// assert!(!changed);
/// # }
/// ```
pub async fn compress_messages_keep_last(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    keep_last: usize,
    reason: &str,
) -> Result<bool> {
    if messages.len() <= keep_last {
        return Ok(false);
    }

    let split_idx =
        codetether_context_policy::selection::active_tail::active_tail_start(messages, keep_last);
    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(messages);

    let context = messages_to_rlm_context(&prefix);
    let ctx_window = context_window_for_model(model);
    if context.trim().is_empty() {
        *messages = prefix;
        messages.extend(tail);
        return Ok(false);
    }
    if let Some(summary) = codetether_rlm_foreground::context_summary(
        &context,
        ctx_window,
        &ctx.session_id,
        reason,
        model,
        Arc::clone(&provider),
        &ctx.rlm_config,
    )
    .await
    {
        super::install::summary(messages, tail, summary, &prefix);
        return Ok(true);
    }
    Ok(false)
}
