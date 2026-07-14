//! Reject out-of-order background recall writes.

use super::indexed_session::IndexedSession;

pub(super) async fn stale(candidate: &IndexedSession) -> bool {
    super::session_io::read(&candidate.session_id)
        .await
        .is_some_and(|current| {
            current.updated_at > candidate.updated_at
                || (current.updated_at == candidate.updated_at
                    && current.message_count > candidate.message_count)
        })
}
