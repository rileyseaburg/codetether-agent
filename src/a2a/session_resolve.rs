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
fn usable_as_session_id(context_id: &str) -> bool {
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
/// * `Some(id)` unsafe as a filename, or `None` → a brand-new random session.
///
/// # Errors
///
/// Returns an error only if creating a new session fails. A failed load of
/// an otherwise-valid id falls through to a new session pinned to that id.
pub async fn resolve_session(context_id: Option<&str>) -> Result<Session> {
    let Some(id) = context_id.filter(|id| usable_as_session_id(id)) else {
        return Session::new().await;
    };

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
mod tests {
    use super::usable_as_session_id;

    #[test]
    fn accepts_uuid_like_ids() {
        assert!(usable_as_session_id("a1b2c3d4-0000-1111-2222-333344445555"));
        assert!(usable_as_session_id("ctx_42-abc"));
    }

    #[test]
    fn rejects_unsafe_ids() {
        assert!(!usable_as_session_id(""));
        assert!(!usable_as_session_id("../escape"));
        assert!(!usable_as_session_id("has space"));
        assert!(!usable_as_session_id(&"x".repeat(129)));
    }
}
