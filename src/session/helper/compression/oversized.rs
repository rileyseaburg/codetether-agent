//! Oversized-last-message compression for the derivation pipeline.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::{ContentPart, Message};
use crate::rlm::RlmChunker;
use crate::session::helper::token::context_window_for_model;

use super::context::CompressContext;
use super::oversized_text::{build_replacement, extract_message_text};

/// Ratio of the model's context window a single "last" message can occupy
/// before [`compress_last_message_if_oversized`] replaces it with an RLM
/// summary.
const OVERSIZED_LAST_MESSAGE_RATIO: f64 = 0.35;

/// Compress the **last** message in `messages` in-place when its text
/// content exceeds [`OVERSIZED_LAST_MESSAGE_RATIO`] of the model's
/// context window.
///
/// The derivation pipeline calls this helper on a clone, so the original
/// user text remains in [`Session::messages`](crate::session::Session)
/// while the LLM sees the compressed projection.
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
pub(crate) async fn compress_last_message_if_oversized(
    messages: &mut [Message],
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
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

    tracing::info!(
        msg_tokens,
        threshold,
        ctx_window,
        "RLM: Last message exceeds context threshold, compressing"
    );

    let replacement = build_replacement(&original, ctx, provider, model, threshold);

    last.content = vec![ContentPart::Text { text: replacement }];
    Ok(true)
}
