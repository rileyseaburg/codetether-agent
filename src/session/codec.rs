//! Native field bindings for shared session JSON/header encoding.
use crate::session::Session;
pub(crate) fn header(session: &Session) -> codetether_session_codec::HeaderView<'_> {
    codetether_session_codec::HeaderView {
        id: &session.id,
        title: &session.title,
        created_at: &session.created_at,
        updated_at: &session.updated_at,
        metadata: &session.metadata,
        agent: &session.agent,
        usage: &session.usage,
    }
}
