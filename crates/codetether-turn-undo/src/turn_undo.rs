//! Truncate corresponding user turns and page metadata.
use crate::turn_undo_count::{count_session_users, count_ui_users};
use crate::turn_undo_index::{session_cut, ui_cut};
use codetether_chat_model::ChatMessage;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::{Message, Role};

/// Remove up to the requested number of complete user turns from both histories.
/// # Arguments
/// * `messages` - Persisted conversation history.
/// * `pages` - Page metadata aligned with conversation messages.
/// * `ui_messages` - Displayed chat history.
/// * `requested` - Maximum number of user turns to remove.
/// # Returns
/// Number removed, limited by the smaller history's user-message count.
/// # Examples
/// ~~~
/// let removed = codetether_turn_undo::truncate_last_turns(&mut vec![], &mut vec![], &mut vec![], 1);
/// assert_eq!(removed, 0);
/// ~~~
pub fn truncate_last_turns(
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
    requested.min(session_count.min(ui_count))
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
