//! Entry builder for typed agent speech messages.

use super::entry_parts::EntryParts;

pub(super) fn speech(
    act: &str,
    from: &str,
    to: &str,
    conversation_id: &str,
    content: &str,
) -> EntryParts {
    let (kind, summary, detail, color) =
        crate::tui::bus_log_entry_payload::speech(act, from, to, conversation_id, content);
    EntryParts::new(kind, summary, detail, color)
}
