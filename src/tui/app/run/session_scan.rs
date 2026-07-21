use crate::session::TailLoad;

/// Selects the startup session policy for the TUI.
///
/// TUI launches begin fresh unless an exact durable session is requested.
pub(super) async fn load(session_id: Option<&str>) -> anyhow::Result<TailLoad> {
    let Some(session_id) = session_id else {
        return Err(anyhow::anyhow!("requested fresh session"));
    };
    crate::session::Session::load_tail(
        session_id,
        crate::tui::app::resume_window::session_resume_window(),
    )
    .await
}
