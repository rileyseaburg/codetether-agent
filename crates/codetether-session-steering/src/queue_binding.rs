//! Queue a follow-up from the current native composer state.
/// Declare the queued-prompt UI helper with the existing preview formatter.
/// # Usage
/// ```text
/// codetether_session_steering::define_app_prompt_queue!(App, truncate_preview);
/// ```
#[macro_export]
macro_rules! define_app_prompt_queue {
    ($app:ty, $preview:path) => {
        pub fn store(app: &mut $app, prompt: ::std::string::String) {
            if prompt.trim().is_empty() {
                return;
            }
            let preview = $preview(&prompt, 80);
            $crate::follow_up::store(prompt);
            app.state.clear_input();
            app.state.status = format!("Queued next prompt: {preview}");
        }
    };
}
