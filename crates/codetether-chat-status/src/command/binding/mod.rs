//! Frontend state and session selection for status commands.
mod host;
/// Declare status reporting over existing frontend and session fields.
/// # Examples
/// See the frontend status-command module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_status_command {
    ($app:path, $session:path, $message:path, $kind:path $(,)?) => {
        use ::std::path::Path;
        use $app as App;
        use $session as Session;
        $crate::__frontend_status_command_host!($message, $kind);
        /// Publish a status report using shared command policy.
        pub(super) async fn show(app: &mut App, cwd: &Path, session: &Session, label: &str) {
            $crate::command::show(&mut host::Native { app, session }, cwd, label).await;
        }
    };
}
