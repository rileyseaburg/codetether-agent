//! Native explicit-session scan using the existing resume-window policy.
use crate::session::TailLoad;
pub(super) async fn load(session_id: Option<&str>) -> anyhow::Result<TailLoad> {
    codetether_tui_startup::resolve::scan::<super::session_resolve_host::Native>(
        session_id,
        crate::tui::app::resume_window::session_resume_window,
    )
    .await
}
