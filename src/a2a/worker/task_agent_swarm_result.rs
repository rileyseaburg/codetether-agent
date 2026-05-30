//! Swarm execution result mapping.

pub(super) fn map_swarm_result(
    text: String,
    success: bool,
    session_id: String,
) -> (&'static str, Option<String>, Option<String>, Option<String>) {
    if success {
        ("completed", Some(text), None, Some(session_id))
    } else {
        (
            "failed",
            Some(text),
            Some("Swarm execution completed with failures".into()),
            Some(session_id),
        )
    }
}
