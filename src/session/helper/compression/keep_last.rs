//! Keep-last prefix compression: buffer core + owning-session wrapper.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::Message;
use crate::session::helper::error::messages_to_rlm_context;
use crate::session::helper::token::context_window_for_model;

use super::context::CompressContext;

/// Compress everything older than the last `keep_last` messages in
/// `messages` into a single synthetic `[AUTO CONTEXT COMPRESSION]`
/// assistant message, in-place.
///
/// This is the `&mut Vec<Message>` core that powers both the legacy
/// [`compress_history_keep_last`] wrapper (which still takes
/// `&mut Session`) and the Phase B
/// [`derive_context`](crate::session::context::derive_context) pipeline
/// (which runs on a history clone).
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
pub(crate) async fn compress_messages_keep_last(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    keep_last: usize,
    reason: &str,
) -> Result<bool> {
    if messages.len() <= keep_last {
        return Ok(false);
    }

    let split_idx = crate::session::context::active_tail::active_tail_start(messages, keep_last);
    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(messages);

    let context = messages_to_rlm_context(&prefix);
    let ctx_window = context_window_for_model(model);
    if context.trim().is_empty() {
        *messages = prefix;
        messages.extend(tail);
        return Ok(false);
    }
    if let Some(summary) = crate::session::helper::compression::compression_defer::context_summary(
        &context,
        ctx_window,
        &ctx.session_id,
        reason,
        model,
        Arc::clone(&provider),
        &ctx.rlm_config,
    ) {
        let toc = crate::session::helper::compression::dropped_toc::render_toc(&prefix, 0);
        crate::session::helper::compression::compression_summary::install(messages, tail, summary, &toc);
        return Ok(true);
    }
    Ok(false)
}
