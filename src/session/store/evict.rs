//! Native disjoint-field view for shared working-set eviction.
use crate::session::Session;
pub(crate) fn before_append(session: &mut Session) {
    codetether_session_store::window::before_append(codetether_session_store::window::WorkingSet {
        id: &session.id,
        state: &session.storage,
        messages: &mut session.messages,
        pages: &mut session.pages,
        tools: &mut session.tool_uses,
        summaries: &mut session.summary_index,
    });
}
