//! Forward native chat clipboard actions to the shared workflow.
use crate::tui::app::state::App;
pub(super) fn handle_copy_reply(app: &mut App) {
    codetether_chat_clipboard::handle_copy_reply(&mut super::chat_clipboard_host::Native(app));
}
