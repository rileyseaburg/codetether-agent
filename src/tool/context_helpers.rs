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

/// Injected field naming the durable session that issued the tool call.
pub(super) const SESSION_ID_FIELD: &str = "__ct_session_id";

/// Load the session that issued this tool call.
///
/// Returns `Ok(None)` when no session exists, `Ok(Some(session))`
/// on success, or `Err` on I/O/parse failures.
pub async fn load_calling_session(args: &Value) -> Result<Option<Session>> {
    let injected = args
        .get(SESSION_ID_FIELD)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(String::from)
        .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok());
    match injected {
        Some(id) => classify(Session::load(&id).await),
        None => load_latest_session().await,
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

fn classify(result: Result<Session>) -> Result<Option<Session>> {
    match result {
        Ok(s) => Ok(Some(s)),
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            if msg.contains("no session")
                || msg.contains("not found")
                || msg.contains("no such file")
            {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
#[path = "context_helpers_tests.rs"]
mod tests;
