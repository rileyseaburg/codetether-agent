//! Frontend approval status, input and chat presentation effects.
/// Implementation detail of [crate::declare_frontend_approval_command].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_approval_command_host {
    ($message:path, $kind:path) => {
        mod host {
            use super::App;
            use $kind as MessageType;
            use $message as ChatMessage;
            pub(super) struct Native<'a>(pub &'a mut App);
            impl $crate::command::View for Native<'_> {
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn status(&self) -> &str {
                    &self.0.state.status
                }
                fn set_waiting(&mut self, waiting: bool) {
                    self.0.state.approval_waiting = waiting;
                }
                fn reset_preview_scroll(&mut self) {
                    self.0.state.approval_preview_scroll = 0;
                }
                fn clear_input(&mut self) {
                    self.0.state.composer.clear_input();
                }
                fn push_system(&mut self, text: String) {
                    self.0
                        .state
                        .messages
                        .push(ChatMessage::new(MessageType::System, text));
                }
                fn scroll_to_bottom(&mut self) {
                    self.0.state.chat_viewport.scroll_to_bottom();
                }
            }
        }
    };
}
