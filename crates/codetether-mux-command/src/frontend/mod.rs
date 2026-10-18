//! Frontend mux result presentation and command entry.
mod output;
/// Declare frontend handling for mux commands and their formatted results.
/// # Examples
/// See the frontend mux command module for concrete state and message types.
#[macro_export]
macro_rules! declare_frontend_mux_command {
    ($app:path, $message:path, $kind:path $(,)?) => {
        use ::std::path::Path;
        use $app as App;
        $crate::__frontend_mux_output!($message, $kind);
        /// Handle mux commands before a chat session is borrowed.
        pub(super) async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
            let Some(result) = $crate::run(cwd, prompt).await else {
                return false;
            };
            output::show(app, result);
            app.state.clear_input();
            true
        }
    };
}
