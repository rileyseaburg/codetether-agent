//! Composer and picker operations supplied by the interactive client.
use std::path::Path;
/// Client state needed for file attachments.
/// # Implementors
/// Native UIs map these operations to their existing composer and notices.
pub trait View {
    /// Borrow the current composer text.
    fn input(&mut self) -> &mut String;
    /// Set the character-based input cursor.
    fn set_cursor(&mut self, cursor: usize);
    /// Enter editing mode.
    fn start_editing(&mut self);
    /// Clear the selected history entry.
    fn clear_history_selection(&mut self);
    /// Refresh command suggestions after changing input.
    fn refresh_suggestions(&mut self);
    /// Select the chat view.
    fn show_chat(&mut self);
    /// Scroll to the latest chat content.
    fn scroll_to_bottom(&mut self);
    /// Append a system notice, set status and scroll using existing behavior.
    fn notice(&mut self, message: String, status: &str);
    /// Open the file picker when no command path is supplied.
    fn open_picker(&mut self, workspace: &Path);
}
