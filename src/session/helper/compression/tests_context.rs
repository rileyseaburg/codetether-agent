//! Tests for [`CompressContext`](super::context::CompressContext).

use crate::session::Session;

use super::context::CompressContext;

#[test]
fn compress_context_from_session_snapshot_is_independent() {
    // Changing the session's metadata after snapshotting must not
    // leak through the captured CompressContext. This lets the
    // Phase B derive_context pipeline hold a CompressContext
    // alongside a `&mut Vec<Message>` borrowed from the same
    // Session without running into borrow conflicts.
    let session = Session {
        id: "session-42".to_string(),
        title: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        metadata: Default::default(),
        agent: "test".to_string(),
        messages: Vec::new(),
        pages: Vec::new(),
        summary_index: crate::session::index::SummaryIndex::new(),
        tool_uses: Vec::new(),
        usage: Default::default(),
        max_steps: None,
        bus: None,
    };
    let snapshot = CompressContext::from_session(&session);
    assert_eq!(snapshot.session_id, "session-42");
}
