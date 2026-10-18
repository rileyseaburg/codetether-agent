//! Bind native session fields and workspace policy to recall projection construction.
use crate::session::Session;
pub(super) fn session(session: &Session) -> Option<super::indexed_session::IndexedSession> {
    let workspace = super::paths::canonical(session.metadata.directory.as_deref()?);
    Some(codetether_recall_index::project(
        codetether_recall_index::ProjectionInput {
            messages: &session.messages,
            session_id: &session.id,
            title: session.title.as_deref(),
            workspace,
            updated_at: session.updated_at,
            message_offset: session.message_offset(),
            message_count: session.message_count(),
        },
    ))
}
