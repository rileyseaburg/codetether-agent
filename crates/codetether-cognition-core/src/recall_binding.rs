//! Frontend message presentation for project recall.
/// Bind project recall to the existing frontend's chat messages and status.
/// # Examples
/// ```text
/// codetether_cognition_core::declare_recall_view!(App, ChatMessage, MessageType);
/// ```
#[macro_export]
macro_rules! declare_recall_view {
    ($app:ty, $message:path, $kind:path $(,)?) => {
        mod recall_view {
            use $kind as MessageType;
            use $message as ChatMessage;
            impl $crate::palace::command::View for $app {
                fn push_system_message(&mut self, text: String) {
                    self.state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll_to_bottom(&mut self) {
                    self.state.chat_viewport.scroll_to_bottom();
                }
                fn set_status(&mut self, text: String) {
                    self.state.status = text;
                }
            }
        }
    };
}
