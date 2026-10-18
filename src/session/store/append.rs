//! Native disjoint-field view for shared transcript append.
use crate::session::Session;
pub(crate) fn message(session: &mut Session, message: crate::provider::Message) {
    codetether_session_store::window::append(
        codetether_session_store::window::WorkingSet {
            id: &session.id,
            state: &session.storage,
            messages: &mut session.messages,
            pages: &mut session.pages,
            tools: &mut session.tool_uses,
            summaries: &mut session.summary_index,
        },
        message,
    );
}
