//! Chat view operations used to switch to a persisted fork.
use codetether_chat_model::ChatMessage;
/// Interactive state operations used by a fork command.
/// # Implementors
/// Native clients map these methods to their existing chat state.
pub trait View {
    /// Whether a response is currently running.
    fn processing(&self) -> bool;
    /// Return the displayed message history.
    fn messages(&self) -> &[ChatMessage];
    /// Replace displayed history with the retained prefix.
    fn set_messages(&mut self, messages: Vec<ChatMessage>);
    /// Set the active session identity.
    fn set_session_id(&mut self, id: String);
    /// Clear incomplete response text.
    fn clear_streaming_text(&mut self);
    /// Clear request timing for the previous session.
    fn clear_request_timing(&mut self);
    /// Scroll to the retained conversation's end.
    fn scroll_to_bottom(&mut self);
    /// Select the chat view.
    fn show_chat(&mut self);
    /// Replace the status line.
    fn set_status(&mut self, status: String);
    /// Append a system notice through the client's existing message path.
    fn push_system_message(&mut self, message: String);
}
