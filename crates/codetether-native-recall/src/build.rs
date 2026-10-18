//! Compact recall projections with absolute transcript provenance.
use crate::Session;
use codetether_recall_index::{IndexedSession, ProjectionInput};
/// Project the loaded session window when its workspace is available.
/// # Arguments
/// * `session` - Application transcript and durable message positions.
/// # Returns
/// An indexed projection, or None for a session without a workspace.
/// # Examples
/// `session(&loaded)` retains absolute offsets even for a bounded tail.
pub fn session<S: Session>(session: &S) -> Option<IndexedSession> {
    let workspace = codetether_recall_store::paths::canonical(session.workspace()?);
    Some(codetether_recall_index::project(ProjectionInput {
        messages: codetether_recall_context::Source::messages(session),
        session_id: codetether_recall_context::Source::id(session),
        title: session.title(),
        workspace,
        updated_at: session.updated_at(),
        message_offset: session.message_offset(),
        message_count: session.message_count(),
    }))
}
