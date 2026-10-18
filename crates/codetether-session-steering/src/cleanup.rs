//! Cleanup of discarded steering inputs and their pending wait activity.

/// Remove all state for a completed or aborted prompt run.
/// # Arguments
/// The durable identity of the completed or aborted session.
/// # Returns
/// Unit; queued inputs and their pending activity records are removed.
/// # Examples
/// ```rust
/// codetether_session_steering::clear("session-1234");
/// ```
pub fn clear(session_id: &str) {
    for input in super::queue::discard(session_id) {
        codetether_collaboration_state::parent_activity::acknowledge_steered(
            session_id,
            input.activity_id(),
        );
    }
}
