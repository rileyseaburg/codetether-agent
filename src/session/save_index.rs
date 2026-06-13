//! Bridge between `Session::save` and the listing index append path.
//!
//! Kept as its own file so `persistence.rs` does not have to grow — the
//! per-save hook is one async call and lives here.

use crate::session::Session;
use crate::session::listing::{SessionSummary, index_file};

/// Build a [`SessionSummary`] for `self` and append one line to the
/// listing index. Best-effort, never fails the save.
pub(crate) async fn record_summary_index_after_save(session: &Session) {
    let summary = SessionSummary {
        id: session.id.clone(),
        title: session.title.clone(),
        created_at: session.created_at,
        updated_at: session.updated_at,
        message_count: session.messages.len(),
        agent: session.agent.clone(),
        directory: session.metadata.directory.clone(),
    };
    index_file::append_async(&summary).await;
}
