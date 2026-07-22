//! Cleanup of discarded steering inputs and their pending wait activity.

/// Remove all state for a completed or aborted prompt run.
pub(crate) fn clear(session_id: &str) {
    for input in super::queue::discard(session_id) {
        crate::tool::agent::collaboration_runtime::parent_activity::acknowledge_steered(
            session_id,
            input.activity_id(),
        );
    }
}
