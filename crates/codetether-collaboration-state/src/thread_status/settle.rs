//! Terminal status selection after a child turn finishes.

use super::{ThreadStatus, get, set};

/// Set terminal status without overwriting interrupted, closed or missing children.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// * `response` — Final response text, if any.
/// * `error` — Failure message, if the turn failed.
/// # Returns
/// Returns after preserving a protected status or publishing the terminal turn status.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::turn("docs-child","done",None);
/// ```
pub fn turn(agent_id: &str, response: &str, error: Option<&str>) {
    if matches!(
        get(agent_id),
        ThreadStatus::Interrupted | ThreadStatus::Shutdown | ThreadStatus::NotFound
    ) {
        return;
    }
    let next = match error {
        Some(error) => ThreadStatus::Errored(error.into()),
        None if response.trim().is_empty() => ThreadStatus::Completed(None),
        None => ThreadStatus::Completed(Some(response.into())),
    };
    set(agent_id, next);
}
