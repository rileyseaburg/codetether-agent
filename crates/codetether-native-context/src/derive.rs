//! Native specialization of shared legacy context derivation.
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Derive an ephemeral [`DerivedContext`] from `session`'s pure history.
///
/// The canonical the source message buffer buffer is never touched —
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
/// [`compress_messages_keep_last`]: codetether_context_compression::compress_messages_keep_last
pub async fn derive_context<S: codetether_context_legacy::Source>(
    session: &S,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    let hooks = super::hooks::native();
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
