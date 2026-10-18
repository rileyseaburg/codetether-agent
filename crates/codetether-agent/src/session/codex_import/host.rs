//! Native parser and session-store bindings for shared Codex imports.
use super::super::Session;
use anyhow::Result;
use async_trait::async_trait;
use codetether_codex_import::Host;
use std::path::{Path, PathBuf};
pub(super) struct ApplicationHost;
#[async_trait]
impl Host for ApplicationHost {
    type Session = Session;
    fn parse(path: &Path, title_override: Option<&str>) -> Result<Session> {
        super::parse::parse_codex_session_from_path(path, title_override)
    }
    fn sessions_dir() -> Result<PathBuf> {
        crate::config::Config::data_dir()
            .map(|dir| dir.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
    }
    fn session_id(session: &Session) -> &str {
        &session.id
    }
    async fn persist(session: Session, path: &Path) -> Result<bool> {
        crate::session::store::imported::persist(session, path).await
    }
    async fn resume(id: &str) -> Result<Session> {
        Session::resume(id).await
    }
}
