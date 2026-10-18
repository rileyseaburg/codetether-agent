//! Frontend chat messages and status updates for inbox notices.
/// Implementation detail of [crate::declare_frontend_inbox].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_inbox_presentation {
    ($message:path, $kind:path) => {
        mod presentation {
            use super::App;
            use $kind as MessageType;
            use $message as ChatMessage;
            impl $crate::Presentation for App {
                fn push_system(&mut self, text: String) {
                    self.state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll_to_bottom(&mut self) {
                    self.state.scroll_to_bottom();
                }
                fn set_status(&mut self, status: String) {
                    self.state.status = status;
                }
            }
        }
    };
}
