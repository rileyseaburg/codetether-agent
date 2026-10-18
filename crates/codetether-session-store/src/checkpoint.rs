//! Acknowledge mutation identities only after a successful write or consistent read.
use crate::Snapshot;
/// Acknowledge current mutation identities and mark all tracked vectors clean.
/// Call only after a successful commit or a consistent database read.
/// # Arguments
/// `session` supplies the shared checkpoint and tracked vectors.
/// # Returns
/// Unit after recording the current vector versions and clearing dirty offsets.
/// # Panics
/// Panics if the checkpoint mutex is poisoned.
/// # Examples
/// ```rust
/// # fn example<S: codetether_session_store::Snapshot>(session: &S) {
/// codetether_session_store::clean(session);
/// assert_eq!(session.messages().dirty_from(), usize::MAX);
/// # }
/// ```
pub fn clean<S: Snapshot>(session: &S) {
    session.state().0.lock().unwrap().versions = [
        session.messages().version(),
        session.pages().version(),
        session.tools().version(),
    ];
    session.messages().clean();
    session.pages().clean();
    session.tools().clean();
}
