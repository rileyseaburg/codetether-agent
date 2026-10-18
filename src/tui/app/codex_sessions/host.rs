//! Bind shared session-selection operations to application storage and UI.
mod session;
mod view;
use crate::session::Session;
use codetether_session_resume::{interactive::Host, loading::LoadedSession};
use std::path::Path;
pub(super) struct Native;
impl Host for Native {
    type Session = Session;
    async fn load(id: &str) -> anyhow::Result<LoadedSession<Session>> {
        crate::tui::app::session_loader::load_session_for_tui(id).await
    }
    async fn hydrate(id: &str) {
        crate::tool::agent::persistence::hydrate_best_effort(id).await;
    }
    async fn import_workspace(cwd: &Path) -> anyhow::Result<(usize, usize)> {
        let report = crate::session::import_codex_sessions_for_directory(cwd).await?;
        Ok((report.imported, report.skipped))
    }
}
