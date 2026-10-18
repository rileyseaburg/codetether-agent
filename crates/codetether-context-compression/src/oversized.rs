//! Oversized-last-message compression for the derivation pipeline.

use std::sync::Arc;

use anyhow::Result;

use codetether_provider_core::limits::context_window_for_model;
use codetether_provider_core::{ContentPart, Message};
use codetether_rlm::RlmChunker;

use super::context::CompressContext;
use super::{replacement::build_replacement, text::extract_message_text};

/// Ratio of the model's context window a single "last" message can occupy
/// before [`compress_last_message_if_oversized`] replaces it with an RLM
/// summary.
const OVERSIZED_LAST_MESSAGE_RATIO: f64 = 0.35;

/// Compress the **last** message in `messages` in-place when its text
/// content exceeds [`OVERSIZED_LAST_MESSAGE_RATIO`] of the model's
/// context window.
///
/// The derivation pipeline calls this helper on a clone, so the original
/// user text remains in the source transcript while the LLM sees the projection.
///
/// # Returns
///
/// `Ok(true)` when the last message was rewritten, `Ok(false)` when no
/// compression was needed.
///
/// # Errors
///
/// Never returns `Err` in the current implementation — RLM failures
/// recover via chunk-based compression, which is infallible. The
/// `Result` shape is preserved for future pipeline wiring.
/// # Arguments
/// * `messages` — Mutable buffer or projection to compress.
/// * `ctx` — Owned session ID and RLM settings.
/// * `provider` — Caller provider used for summarization.
/// * `model` — Caller model used to derive context limits.
/// * `resolver` — Native model resolver for background summaries.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, resolver: codetether_rlm_background::Resolver) {
/// let ctx = codetether_context_compression::CompressContext::new("session", &Default::default());
/// let mut messages = Vec::new();
/// let changed = codetether_context_compression::compress_last_message_if_oversized(&mut messages, &ctx, provider, "model", resolver).await.unwrap();
/// assert!(!changed);
/// # }
/// ```
pub async fn compress_last_message_if_oversized(
    messages: &mut [Message],
    ctx: &CompressContext,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    resolver: codetether_rlm_background::Resolver,
) -> Result<bool> {
    let Some(last) = messages.last_mut() else {
        return Ok(false);
    };

    let original = extract_message_text(last);
    if original.is_empty() {
        return Ok(false);
    }

    let ctx_window = context_window_for_model(model);
    let msg_tokens = RlmChunker::estimate_tokens(&original);
    let threshold = (ctx_window as f64 * OVERSIZED_LAST_MESSAGE_RATIO) as usize;
    if msg_tokens <= threshold {
        return Ok(false);
    }

    tracing::info!(target: "codetether_agent::session::helper::compression::oversized",
        msg_tokens,
        threshold,
        ctx_window,
        "RLM: Last message exceeds context threshold, compressing"
    );

    let replacement = build_replacement(&original, ctx, provider, model, threshold, resolver);

    last.content = vec![ContentPart::Text { text: replacement }];
    Ok(true)
}
