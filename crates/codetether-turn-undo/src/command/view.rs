//! Interactive view access for undo guards and result presentation.
use codetether_chat_model::ChatMessage;
/// View operations needed by interactive history undo.
/// # Implementors
/// Interactive clients bind their existing chat state and notice behavior.
pub trait View {
    /// Whether an active response prevents safe undo.
    fn processing(&self) -> bool;
    /// Borrow the displayed message history.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Clear any retained streaming response text.
    fn clear_streaming_text(&mut self);
    /// Move the viewport to the end of the retained history.
    fn scroll_to_bottom(&mut self);
    /// Replace the command status message.
    fn set_status(&mut self, status: String);
    /// Append a system notice using the client's existing behavior.
    fn push_system_message(&mut self, message: String);
}
