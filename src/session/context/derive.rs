//! Native specialization of shared legacy context derivation.
use super::helpers::DerivedContext;
use crate::provider::ToolDefinition;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

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
/// * `force_keep_last` — When `Some(n)`, first force a single
///   [`compress_messages_keep_last`] call, then still enforce the budget.
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
    let hooks = codetether_context_legacy::Hooks {
        apply: crate::session::helper::experimental::apply_all,
        repair: crate::session::helper::experimental::pairing::repair_orphans,
        resolver: crate::session::helper::rlm_background::Background::resolve_model,
    };
    codetether_context_legacy::derive_context(
        session,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        force_keep_last,
        hooks,
    )
    .await
}
