//! Frontend composer and chat presentation for review preparation.
/// Implementation detail of [crate::declare_frontend_utility].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_utility_review {
    ($message:path, $kind:path) => {
        mod review_host {
            use super::App;
            use $kind as MessageType;
            use $message as ChatMessage;
            impl $crate::utility::review::View for App {
                fn clear_input(&mut self) {
                    self.state.clear_input();
                }
                fn input(&mut self) -> &mut String {
                    &mut self.state.input
                }
                fn cursor(&mut self) -> &mut usize {
                    &mut self.state.input_cursor
                }
                fn set_status(&mut self, status: String) {
                    self.state.status = status;
                }
                fn push_system(&mut self, text: String) {
                    self.state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll_to_bottom(&mut self) {
                    self.state.scroll_to_bottom();
                }
            }
        }
    };
}
