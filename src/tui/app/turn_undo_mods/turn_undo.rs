use crate::provider::{Message, Role};
use crate::session::pages::PageKind;
use crate::tui::app::turn_undo_mods::turn_undo_count::{count_session_users, count_ui_users};
use crate::tui::app::turn_undo_mods::turn_undo_index::{session_cut, ui_cut};
use crate::tui::chat::message::ChatMessage;

pub(crate) fn truncate_last_turns(
    messages: &mut Vec<Message>,
    pages: &mut Vec<PageKind>,
    ui_messages: &mut Vec<ChatMessage>,
    requested: usize,
) -> usize {
    let undo_count = undo_count(messages, ui_messages, requested);
    if undo_count == 0 {
        return 0;
    }
    truncate_session(messages, pages, undo_count);
    truncate_ui(ui_messages, undo_count);
    undo_count
}

fn undo_count(messages: &[Message], ui_messages: &[ChatMessage], requested: usize) -> usize {
    let session_count = count_session_users(messages);
    let ui_count = count_ui_users(ui_messages);
    requested.min(session_count).max(requested.min(ui_count))
}

fn truncate_session(messages: &mut Vec<Message>, pages: &mut Vec<PageKind>, count: usize) {
    if let Some(cut) = session_cut(messages, count) {
        messages.truncate(cut);
        pages.truncate(cut.min(pages.len()));
    }
}

fn truncate_ui(messages: &mut Vec<ChatMessage>, count: usize) {
    if let Some(cut) = ui_cut(messages, count) {
        messages.truncate(cut);
    }
}

pub(crate) fn is_user_message(message: &Message) -> bool {
    message.role == Role::User
}
