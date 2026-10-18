//! Supply native UI context for shared approval reviewer dispatch.
use crate::tui::app::state::{App, approval_queue::ApprovalSnapshot};
pub(super) fn start(app: &mut App, item: &ApprovalSnapshot) {
    crate::review::dispatch::start(item, || crate::review::dispatch::Context {
        workspace: std::path::PathBuf::from(&app.state.cwd_display),
        session_id: app.state.session_id.clone(),
        session_model: app.state.last_completion_model.clone(),
    });
}
