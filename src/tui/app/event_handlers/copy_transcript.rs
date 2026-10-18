//! Forward native chat clipboard actions to the shared workflow.
use crate::tui::app::state::App;
pub(crate) use codetether_chat_transcript::build_transcript;
pub(super) fn handle_copy_transcript(app: &mut App) {
    codetether_chat_clipboard::handle_copy_transcript(&mut super::chat_clipboard_host::Native(app));
}
