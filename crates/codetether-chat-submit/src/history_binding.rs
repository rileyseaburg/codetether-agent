//! Frontend input and prompt-history capture.
/// Bind raw prompt capture to existing frontend history storage.
/// # Examples
/// ```text
/// codetether_chat_submit::declare_frontend_history!(App);
/// ```
#[macro_export]
macro_rules! declare_frontend_history {
    ($app:path $(,)?) => {
        use $app as App;
        impl $crate::history::History for App {
            fn input(&self) -> &str {
                &self.state.composer.input
            }
            fn push_history(&mut self, prompt: String) {
                self.state.composer.push_history(prompt);
            }
        }
        pub(super) fn capture(app: &mut App) -> String {
            $crate::history::capture(app)
        }
    };
}
