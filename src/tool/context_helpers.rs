//! Shared session loader for context tools.
//!
//! Context tools operate on the **calling** session. The runtime injects
//! `__ct_session_id` into every tool input, so the exact durable session is
//! loaded by ID. Several mux sessions may share one checkout (`--no-worktree`),
//! so a workspace-scoped "latest session" scan is used only for callers that
//! carry no session identity, never as the primary path.

use crate::session::Session;
use anyhow::Result;
use serde_json::Value;
mod errors;
use errors::classify;

/// Injected field naming the durable session that issued the tool call.
pub(super) const SESSION_ID_FIELD: &str = "__ct_session_id";

/// Load the session that issued this tool call.
///
/// Returns `Ok(None)` when no session exists, `Ok(Some(session))`
/// on success, or `Err` on I/O/parse failures.
pub async fn load_calling_session(args: &Value) -> Result<Option<Session>> {
    load_calling_window(args, usize::MAX).await
}

/// Load only a bounded working window of the calling session.
pub(super) async fn load_calling_window(args: &Value, window: usize) -> Result<Option<Session>> {
    let injected = args
        .get(SESSION_ID_FIELD)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(String::from)
        .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok());
    match injected {
        Some(id) => classify(
            Session::load_tail(&id, window)
                .await
                .map(|loaded| loaded.session),
        ),
        None => {
            let cwd = std::env::current_dir().ok();
            classify(
                Session::last_for_directory_tail(cwd.as_deref(), window)
                    .await
                    .map(|loaded| loaded.session),
            )
        }
    }
}

/// Load the latest session for the current working directory.
///
/// Prefer [`load_calling_session`]; this cannot distinguish concurrent
/// sessions rooted in the same checkout.
pub async fn load_latest_session() -> Result<Option<Session>> {
    let cwd = std::env::current_dir().ok();
    classify(Session::last_for_directory(cwd.as_deref()).await)
}

#[cfg(test)]
#[path = "context_helpers_tests.rs"]
mod tests;
