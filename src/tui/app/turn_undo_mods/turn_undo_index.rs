use crate::provider::Message;
use crate::tui::app::turn_undo_mods::turn_undo::is_user_message;
use crate::tui::app::turn_undo_mods::turn_undo_count::is_ui_user;
use crate::tui::chat::message::ChatMessage;

pub(crate) fn session_cut(messages: &[Message], count: usize) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, msg)| is_user_message(msg).then_some(idx))
        .nth(count - 1)
}

pub(crate) fn ui_cut(messages: &[ChatMessage], count: usize) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, msg)| is_ui_user(msg).then_some(idx))
        .nth(count - 1)
}
