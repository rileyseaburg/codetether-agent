//! Frontend approval-command state and dispatch bindings.
mod host;
mod release;
/// Declare approval-command dispatch using the existing frontend view state.
/// # Examples
/// See the frontend approval-command module for message and state type selection.
#[macro_export]
macro_rules! declare_frontend_approval_command {
    ($app:path, $message:path, $kind:path $(,)?) => {
        use $app as App;
        $crate::__frontend_approval_command_host!($message, $kind);
        /// Dispatch an approval command through shared command policy.
        pub(crate) fn run(app: &mut App, prompt: &str) -> bool {
            $crate::command::run(&mut host::Native(app), prompt)
        }
    };
}
