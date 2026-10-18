//! Presentation operations for the chat shell feature.
use crate::State;
use codetether_chat_model::ChatMessage;
/// Binds shell state to a frontend's transcript and composer.
/// # Implementors
/// Frontends provide the owned [State], message insertion and input updates.
pub trait View {
    /// Borrow the frontend's background command state.
    fn shell(&mut self) -> &mut State;
    /// Append one user or tool-result message.
    fn push_message(&mut self, message: ChatMessage);
    /// Replace the visible status text.
    fn set_status(&mut self, status: String);
    /// Reset the composer after consuming a shell command.
    fn clear_input(&mut self);
    /// Follow the newest transcript entry.
    fn scroll_to_bottom(&mut self);
}
