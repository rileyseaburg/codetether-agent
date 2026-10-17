//! Named lifecycle transitions shared by child execution paths.

use super::{ThreadStatus, set};
use std::fmt::Display;

/// Mark a child turn as running.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// Returns after recording the requested lifecycle transition.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::running("docs-child");
/// ```
pub fn running(agent_id: &str) {
    set(agent_id, ThreadStatus::Running);
}

/// Mark a child turn as interrupted.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// Returns after recording the requested lifecycle transition.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::interrupted("docs-child");
/// ```
pub fn interrupted(agent_id: &str) {
    set(agent_id, ThreadStatus::Interrupted);
}

/// Mark a child as closed.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// Returns after recording the requested lifecycle transition.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::shutdown("docs-child");
/// ```
pub fn shutdown(agent_id: &str) {
    set(agent_id, ThreadStatus::Shutdown);
}

/// Mark a restored child idle with no completion text.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// # Returns
/// Returns after recording the requested lifecycle transition.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// codetether_collaboration_state::thread_status::restored("docs-child");
/// ```
pub fn restored(agent_id: &str) {
    set(agent_id, ThreadStatus::Completed(None));
}

/// Publish a failure status when the supplied result is an error.
/// # Arguments
/// * `agent_id` — Canonical child session ID.
/// * `result` — Result to inspect and return without alteration.
/// # Returns
/// The supplied result, after publishing an error status if it failed.
/// # Errors
/// Returns the original error unchanged.
/// # Panics
/// Panics if the status registry lock is poisoned.
/// # Examples
/// ```
/// let _:Result<(),String>=codetether_collaboration_state::thread_status::track_error("docs-child",Ok(()));
/// ```
pub fn track_error<T, E: Display>(agent_id: &str, result: Result<T, E>) -> Result<T, E> {
    if let Err(error) = &result {
        set(agent_id, ThreadStatus::Errored(error.to_string()));
    }
    result
}
