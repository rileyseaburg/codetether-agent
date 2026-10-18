//! Reject out-of-order background recall writes.

use codetether_recall_index::IndexedSession;

pub(super) async fn stale<H: crate::Host>(candidate: &IndexedSession, host: &H) -> bool {
    super::session_io::read(&candidate.session_id, host)
        .await
        .is_some_and(|current| {
            current.updated_at > candidate.updated_at
                || (current.updated_at == candidate.updated_at
                    && current.message_count > candidate.message_count)
        })
}
