//! Frontend view operations for steering.
/// Implementation detail of [crate::declare_frontend_steering].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_steering_view {
    ($message:path) => {
        mod view {
            use super::Native;
            use $message as ChatMessage;
            impl $crate::steering::View for Native<'_> {
                fn clear_attachments(&mut self) {
                    self.app.state.attachments.clear();
                }
                fn messages(&mut self) -> &mut Vec<ChatMessage> {
                    &mut self.app.state.messages
                }
                fn clear_input(&mut self) {
                    self.app.state.composer.clear_input();
                }
                fn status(&mut self, message: String) {
                    self.app.state.status = message;
                }
                fn scroll(&mut self) {
                    self.app.state.chat_viewport.scroll_to_bottom();
                }
            }
        }
    };
}
