//! Preserve conversation identity across inbound A2A turns.
use super::*;
use anyhow::Result;

/// Resume a conversation or create it only when persisted state is absent.
///
/// # Arguments
/// * `context_id` - Optional conversation identifier to resume or pin.
/// * `store` - Application storage operations.
/// # Returns
/// A resumed session, a new session pinned to the context, or an unpinned new session.
/// # Errors
/// Returns invalid-ID, session creation, existence-check or existing-session loading errors.
/// # Examples
/// ```rust,no_run
/// # async fn demo<S: codetether_a2a_http::conversation::ConversationStore>(store: &S) -> anyhow::Result<()> {
/// let _session = codetether_a2a_http::conversation::resolve_session(Some("review_42"), store).await?;
/// # Ok(()) }
/// ```
pub async fn resolve_session<S: ConversationStore>(
    context_id: Option<&str>,
    store: &S,
) -> Result<S::Session> {
    let Some(id) = context_id else {
        return store.create().await;
    };
    if !usable_as_session_id(id) {
        anyhow::bail!("A2A context_id must match [A-Za-z0-9_-] and be at most 128 bytes");
    }

    match store.resume(id).await {
        Ok(session) => {
            tracing::debug!(target: "codetether_agent::a2a::session_resolve",context_id = %id, "Resumed A2A conversation session");
            return Ok(session);
        }
        Err(error) if store.exists(id)? => return Err(error),
        Err(_) => {}
    }

    let mut session = store.create().await?;
    store.set_id(&mut session, id);
    tracing::debug!(target: "codetether_agent::a2a::session_resolve",context_id = %id, "Started new A2A conversation session");
    Ok(session)
}
