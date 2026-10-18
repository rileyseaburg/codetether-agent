//! Frontend shell-command entry and output-drain bindings.
mod view;
/// Declare background-shell view effects and completed-output draining.
/// # Examples
/// See the frontend shell background module for its selected message type.
#[macro_export]
macro_rules! declare_frontend_shell_view {
    ($app:path, $message:path $(,)?) => {
        use $app as App;
        $crate::__frontend_shell_view!($app, $message);
        /// Drain a completed command into the visible transcript.
        pub(crate) fn drain_shell_events(app: &mut App) -> bool {
            $crate::drain(app)
        }
    };
}
/// Declare the frontend's asynchronous shell-prefix entry wrapper.
/// # Examples
/// See the frontend shell command module for invocation and visibility.
#[macro_export]
macro_rules! declare_frontend_shell_command {
    ($visibility:vis, $app:path $(,)?) => {
        use $app as App;
        use ::std::path::Path;
        /// Consume shell input through the shared background command lifecycle.
        $visibility async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
            $crate::submit(app, cwd, prompt)
        }
    };
}
