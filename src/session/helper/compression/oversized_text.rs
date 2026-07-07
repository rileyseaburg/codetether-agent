//! Replacement-text construction for oversized last messages.

use std::sync::Arc;

use crate::provider::{ContentPart, Message};
use crate::rlm::RlmChunker;

use super::context::CompressContext;

/// Character budget retained verbatim from the original message as a
/// prefix, after RLM compression, so the model can see the literal
/// opening of the request even when the body was summarised.
const OVERSIZED_LAST_MESSAGE_PREFIX_CHARS: usize = 500;

/// Build the compressed replacement text for an oversized last message:
/// a cached background-RLM summary when available, otherwise a
/// deterministic [`RlmChunker::compress`] projection.
pub(super) fn build_replacement(
    original: &str,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    threshold: usize,
) -> String {
    let msg_tokens = RlmChunker::estimate_tokens(original);
    let cached = crate::session::helper::rlm_background::context_summary(
        original,
        "oversized_last_message",
        &ctx.session_id,
        model,
        provider,
        &ctx.rlm_config,
    );
    cached
        .map(|summary| {
            crate::session::helper::compression::compression_last_message::wrap_cached(
                summary,
                original,
                msg_tokens,
                OVERSIZED_LAST_MESSAGE_PREFIX_CHARS,
            )
        })
        .unwrap_or_else(|| RlmChunker::compress(original, threshold, None))
}

/// Concatenate the textual content of `msg`, skipping non-text parts.
pub(super) fn extract_message_text(msg: &Message) -> String {
    let mut buf = String::new();
    for part in &msg.content {
        if let ContentPart::Text { text } = part {
            if !buf.is_empty() {
                buf.push('\n');
            }
            buf.push_str(text);
        }
    }
    buf
}
