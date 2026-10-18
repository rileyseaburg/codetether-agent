//! Frontend copy command state and presentation bindings.
/// Declare the frontend copy command over existing chat state.
/// # Examples
/// See the frontend utility copy module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_copy_command {
    ($app:path, $message:path $(,)?) => {
        use $app as App;
        use $message as ChatMessage;
        impl $crate::command::View for App {
            fn messages(&self) -> &[ChatMessage] {
                &self.state.messages
            }
            fn set_status(&mut self, status: String) {
                self.state.status = status;
            }
            fn clear_input(&mut self) {
                self.state.composer.clear_input();
            }
        }
        pub(super) fn run(app: &mut App, prompt: &str) {
            $crate::command::run(app, prompt);
        }
    };
}
