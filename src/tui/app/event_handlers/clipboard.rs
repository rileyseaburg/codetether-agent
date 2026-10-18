//! Forward native chat clipboard actions to the shared workflow.
use crate::tui::app::state::App;
pub(super) fn handle_clipboard_paste(app: &mut App) {
    codetether_chat_clipboard::handle_clipboard_paste(&mut super::chat_clipboard_host::Native(app));
}
