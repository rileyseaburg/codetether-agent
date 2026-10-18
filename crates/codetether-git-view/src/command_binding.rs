//! Frontend diff command state and presentation bindings.
/// Declare the frontend diff command over existing chat state.
/// # Examples
/// See the frontend utility diff module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_diff_command {
    ($app:path, $message:path, $kind:path $(,)?) => {
        use ::std::path::Path;
        use $app as App;
        use $kind as MessageType;
        use $message as ChatMessage;
        impl $crate::command::View for App {
            fn clear_input(&mut self) {
                self.state.composer.clear_input();
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
                self.state.chat_viewport.scroll_to_bottom();
            }
        }
        pub(super) fn show(app: &mut App, cwd: &Path) {
            $crate::command::show(app, cwd);
        }
    };
}
