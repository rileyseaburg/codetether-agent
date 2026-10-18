use codetether_chat_model::ChatMessage;
/// Visible UI updates after active-turn steering.
/// # Implementors
/// UI adapters expose their log and existing input/reset methods.
pub trait View {
    /// Clear pending images followed by pending text sidecars.
    fn clear_attachments(&mut self);
    /// Borrow the visible chat message log.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Clear the accepted raw input.
    fn clear_input(&mut self);
    /// Replace the visible status text.
    fn status(&mut self, message: String);
    /// Scroll to the newest chat message.
    fn scroll(&mut self);
}
