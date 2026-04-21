//! Phase-A legacy derivation: clone + experimental + enforce + pairing repair.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::ToolDefinition;
use crate::session::Session;
use crate::session::SessionEvent;
use crate::session::helper::compression::{
    CompressContext, compress_last_message_if_oversized,
};
use crate::session::helper::experimental;

use super::compress_step::run_compression_step;
use super::helpers::{DerivedContext, messages_len_changed};

/// Derive an ephemeral [`DerivedContext`] from `session`'s pure history.
///
/// The canonical [`Session::messages`] buffer is never touched —
/// `session` is borrowed immutably.
///
/// # Arguments
///
/// * `session` — The owning session (read-only borrow).
/// * `provider` — Caller's primary provider.
/// * `model` — Caller's primary model identifier.
/// * `system_prompt` — Included in token estimates.
/// * `tools` — Tool definitions, included in token estimates.
/// * `event_tx` — Optional channel for compaction lifecycle events.
/// * `force_keep_last` — When `Some(n)`, skip the adaptive budget
///   cascade and force a single [`compress_messages_keep_last`] call.
///
/// # Errors
///
/// Propagates any error from the underlying compression pipeline that
/// the recovery cascade cannot absorb.
///
/// [`compress_messages_keep_last`]: crate::session::helper::compression::compress_messages_keep_last
pub async fn derive_context(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    let origin_len = session.messages.len();
    let mut messages = session.messages.clone();
    let ctx = CompressContext::from_session(session);

    let step0 = compress_last_message_if_oversized(
        &mut messages, &ctx, Arc::clone(&provider), model,
    )
    .await?;

    experimental::apply_all(&mut messages);

    let before = messages.len();
    let step1 =
        run_compression_step(&mut messages, &ctx, provider, model, system_prompt, tools, event_tx, force_keep_last)
            .await?;

    experimental::pairing::repair_orphans(&mut messages);

    let compressed = step0 || step1 || messages_len_changed(before, &messages);
    Ok(DerivedContext {
        messages,
        origin_len,
        compressed,
    })
}
