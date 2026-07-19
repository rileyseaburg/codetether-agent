//! Resolve the conversation [`Session`] backing an inbound A2A request.
//!
//! A2A `message/send` carries an optional `context_id` that ties multiple
//! turns into one conversation. Treating every request as a fresh session
//! makes the agent "forget" prior turns; persisting and reloading by
//! `context_id` keeps multi-turn context across requests and restarts.

use crate::session::Session;
use anyhow::Result;

/// Whether a `context_id` is safe to use directly as a session id.
///
/// Mirrors the validation in `Session::session_path`: non-empty, ≤ 128
/// bytes, and only `[A-Za-z0-9_-]` so it cannot escape the sessions
/// directory.
pub(crate) fn usable_as_session_id(context_id: &str) -> bool {
    !context_id.is_empty()
        && context_id.len() <= 128
        && !context_id.contains(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
}

/// Load the conversation for `context_id`, or start a fresh one.
///
/// * `Some(id)` naming an existing on-disk session → loaded so prior turns
///   are remembered.
/// * `Some(id)` with no existing session → a new session pinned to `id` so
///   the *next* turn can find it.
/// * `Some(id)` unsafe as a filename → an error.
/// * `None` → a brand-new random session.
///
/// # Arguments
///
/// * `context_id` — Optional A2A conversation identifier to resume or pin to a
///   new session.
///
/// # Returns
///
/// The persisted session for the conversation, or a new session when the
/// context is omitted or has not been seen before.
///
/// # Errors
///
/// Returns an error for an unsafe context ID or if creating a new session
/// fails. A failed load of a valid ID starts a new session pinned to that ID.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::a2a::session_resolve::resolve_session;
/// let session = resolve_session(Some("forgejo_pr_42_head")).await.unwrap();
/// assert_eq!(session.id, "forgejo_pr_42_head");
/// # });
/// ```
pub async fn resolve_session(context_id: Option<&str>) -> Result<Session> {
    let Some(id) = context_id else {
        return Session::new().await;
    };
    if !usable_as_session_id(id) {
        anyhow::bail!("A2A context_id must match [A-Za-z0-9_-] and be at most 128 bytes");
    }

    if let Ok(session) = Session::load(id).await {
        tracing::debug!(context_id = %id, "Resumed A2A conversation session");
        return Ok(session);
    }

    let mut session = Session::new().await?;
    session.id = id.to_string();
    tracing::debug!(context_id = %id, "Started new A2A conversation session");
    Ok(session)
}

#[cfg(test)]
#[path = "session_resolve_tests.rs"]
mod tests;
