//! Frontend text feedback for pending approval revisions.
/// Declare direct-text approval feedback using existing image recovery and queue lookup.
/// # Examples
/// See the frontend approval-feedback module for callback selection.
#[macro_export]
macro_rules! declare_frontend_approval_feedback {
    ($app:path; $recover:path, $active:path, $command:path $(,)?) => {
        use $app as App;
        /// Sends ordinary chat input as the reason for rejecting and revising a tool call.
        pub(super) fn submit(app: &mut App) -> bool {
            $recover(app);
            let prompt = app.state.input.trim().to_string();
            if prompt.is_empty() || prompt.starts_with('/') {
                return false;
            }
            let Some(id) = $active() else {
                return false;
            };
            $command(app, &format!("/deny {id} {prompt}"))
        }
    };
}
