//! Frontend shell lifecycle state and transcript effects.
/// Implementation detail of [crate::declare_frontend_shell_view].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_shell_view {
    ($app:path, $message:path) => {
        mod view {
            use $app as App;
            use $message as ChatMessage;
            impl $crate::View for App {
                fn shell(&mut self) -> &mut $crate::State {
                    &mut self.state.shell
                }
                fn push_message(&mut self, message: ChatMessage) {
                    self.state.messages.push(message);
                }
                fn set_status(&mut self, status: String) {
                    self.state.status = status;
                }
                fn clear_input(&mut self) {
                    self.state.composer.clear_input();
                }
                fn scroll_to_bottom(&mut self) {
                    self.state.chat_viewport.scroll_to_bottom();
                }
            }
        }
    };
}
