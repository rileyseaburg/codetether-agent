//! Frontend feedback for automatic prompt-to-goal adoption.
/// Declare best-effort goal adoption using the frontend session slot.
/// # Examples
/// See the frontend goal-adoption module for its selected runtime callback.
#[macro_export]
macro_rules! declare_frontend_goal_adoption {
    ($app:path, $slot:path, $adopt:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        /// Adopt `prompt` as the active goal when auto-goal mode is on.
        ///
        /// Failures are surfaced in the status bar and never block submission.
        pub(super) async fn adopt(app: &mut App, slot: &SessionSlot, prompt: &str) {
            let Some(session_id) = slot.borrow().map(|session| session.id.clone()) else {
                return;
            };
            match $adopt(&session_id, prompt).await {
                Ok(true) => app.state.status = "Prompt set as goal; verifier gates completion".into(),
                Ok(false) => {}
                Err(error) => {
                    tracing::warn!(error = %error, "auto-goal adoption failed");
                    app.state.status = format!("Auto-goal failed: {error}");
                }
            }
        }
    };
}
