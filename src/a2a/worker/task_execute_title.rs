//! Session title preseeding for worker-executed tasks.

/// A2A tasks already have a human-readable title. Preseed it so worker
/// execution does not block on the optional, provider-backed title call before
/// the actual task model is invoked. Preserve titles on resumed sessions.
pub(super) fn preseed_task_title(session: &mut crate::session::Session, title: &str) {
    if session.title.is_none() {
        session.set_title(title.to_string());
    }
}
