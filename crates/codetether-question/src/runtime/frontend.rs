//! Snapshot, edit, and dismiss the pending question for an exact session.
use super::store::lock;
use crate::{Action, Draft, Error};
/// Snapshot the pending selection for an exact session.
///
/// # Arguments
/// * `session` — Frontend session ID, never a workspace or latest-session lookup.
/// # Returns
/// The editable batch, or `None` after submission/cancellation.
/// # Examples
/// ```
/// assert!(codetether_question::runtime::pending("missing").is_none());
/// ```
pub fn pending(session: &str) -> Option<Draft> {
    lock()
        .get(session)
        .filter(|p| p.reply.is_some())
        .map(|p| p.draft.clone())
}
/// Apply a frontend action to an exact session's pending question.
///
/// # Arguments
/// * `session` — Session ID.
/// * `action` — Selection/edit action.
/// # Returns
/// False when no unanswered question exists.
/// # Examples
/// ```
/// use codetether_question::{runtime, Action};
/// assert!(!runtime::act("missing", Action::Submit));
/// ```
pub fn act(session: &str, action: Action) -> bool {
    let mut pending = lock();
    let Some(p) = pending.get_mut(session).filter(|p| p.reply.is_some()) else {
        return false;
    };
    if let Some(answers) = p.draft.apply(action) {
        if let Some(reply) = p.reply.take() {
            let _ = reply.send(Ok(answers));
        }
    }
    true
}
/// Dismiss without fabricating an answer.
///
/// # Arguments
/// * `session` — Session ID.
/// # Returns
/// Whether an unanswered question was found.
/// # Examples
/// ```
/// assert!(!codetether_question::runtime::dismiss("missing"));
/// ```
pub fn dismiss(session: &str) -> bool {
    let reply = lock().get_mut(session).and_then(|p| p.reply.take());
    let Some(reply) = reply else {
        return false;
    };
    let _ = reply.send(Err(Error::Dismissed));
    true
}
