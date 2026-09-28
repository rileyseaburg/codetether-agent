//! Auto-goal hook: adopt a plain chat prompt as the session goal.

use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

/// Adopt `prompt` as the active goal when auto-goal mode is on.
///
/// Failures are surfaced in the status bar and never block submission.
pub(super) async fn adopt(app: &mut App, slot: &SessionSlot, prompt: &str) {
    let Some(session_id) = slot.borrow().map(|session| session.id.clone()) else {
        return;
    };
    match crate::session::tasks::runtime::adopt_prompt(&session_id, prompt).await {
        Ok(true) => app.state.status = "Prompt set as goal; verifier gates completion".into(),
        Ok(false) => {}
        Err(error) => {
            tracing::warn!(error = %error, "auto-goal adoption failed");
            app.state.status = format!("Auto-goal failed: {error}");
        }
    }
}
