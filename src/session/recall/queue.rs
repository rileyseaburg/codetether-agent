//! Latest-snapshot coalescing for background recall indexing.

use crate::session::Session;

pub(crate) fn schedule(session: &Session) {
    super::tombstone::restore(&session.id);
    let spawn = super::queue_state::enqueue(session);
    if spawn {
        super::queue_worker::spawn(session.id.clone());
    }
}

pub(crate) async fn remove(session_id: &str) -> anyhow::Result<()> {
    super::tombstone::mark(session_id);
    super::queue_state::remove(session_id);
    super::store::remove(session_id).await
}

pub(super) fn take(session_id: &str) -> Option<Session> {
    super::queue_state::take(session_id)
}

pub(super) fn settle(session_id: &str) -> bool {
    super::queue_state::settle(session_id)
}
