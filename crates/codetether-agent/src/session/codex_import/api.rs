//! Native-host specialization of shared import entry points.
use super::super::Session;
use super::CodexImportReport;
use super::host::ApplicationHost;
use anyhow::Result;
pub async fn import_codex_session_by_id(id: &str) -> Result<Session> {
    codetether_codex_import::import_codex_session_by_id::<ApplicationHost>(id).await
}
pub async fn import_codex_session_path(path: &std::path::Path) -> Result<Session> {
    codetether_codex_import::import_codex_session_path::<ApplicationHost>(path).await
}
pub async fn import_codex_sessions_for_directory(
    dir: &std::path::Path,
) -> Result<CodexImportReport> {
    codetether_codex_import::import_codex_sessions_for_directory::<ApplicationHost>(dir).await
}
pub async fn load_or_import_session(id: &str) -> Result<Session> {
    codetether_codex_import::load_or_import_session::<ApplicationHost>(id).await
}
