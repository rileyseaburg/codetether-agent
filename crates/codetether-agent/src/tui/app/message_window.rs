//! Native session binding for shared retained-message selection.
use crate::{provider::Message, session::Session};
/// Select the retained chronological tail of the session history.
pub fn recent(session: &Session) -> impl DoubleEndedIterator<Item = &Message> {
    codetether_chat_transcript::sync::recent(session.history())
}
