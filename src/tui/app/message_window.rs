//! Live chat window selection.

use crate::provider::Message;
use crate::session::Session;
use crate::tui::retained_payload::CHAT_RETAINED_MAX_ITEMS;

pub fn recent(session: &Session) -> impl DoubleEndedIterator<Item = &Message> {
    session
        .history()
        .iter()
        .rev()
        .take(CHAT_RETAINED_MAX_ITEMS)
        .rev()
}
