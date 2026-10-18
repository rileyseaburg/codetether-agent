//! Native storage adapter for shared bounded session loading.
mod host;
use crate::session::Session;
use crate::tui::app::resume_window::session_resume_window;
/// A native session with bounded-load metadata.
pub type LoadedSession = codetether_session_resume::loading::LoadedSession<Session>;
/// Load the bounded native session or fall back to archive import.
pub async fn load_session_for_tui(id: &str) -> anyhow::Result<LoadedSession> {
    codetether_session_resume::loading::load::<host::Native>(id, session_resume_window()).await
}
