//! Resolve the conversation [`Session`] backing an inbound A2A request.
//!
//! A2A `message/send` carries an optional `context_id` that ties multiple
//! turns into one conversation. Treating every request as a fresh session
//! makes the agent "forget" prior turns; persisting and reloading by
//! `context_id` keeps multi-turn context across requests and restarts.

use crate::session::Session;
use anyhow::Result;
#[cfg(test)]
use codetether_a2a_http::conversation::usable_as_session_id;

struct Store;
#[path = "session_store.rs"]
mod store;

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
/// fails. Failed loads of existing session files remain errors; absent files
/// start a new session pinned to the valid ID.
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
    codetether_a2a_http::conversation::resolve_session(context_id, &Store).await
}

#[cfg(test)]
#[path = "session_resolve_tests.rs"]
mod tests;
