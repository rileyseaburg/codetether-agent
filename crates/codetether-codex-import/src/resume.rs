//! resume import workflow.
use crate::{import_codex_session_by_id, import_codex_session_path};
use anyhow::Result;
use codetether_codex_discovery::{codex_home_dir, find_codex_session_path_by_id};

/// Prefer a discovered Codex archive, otherwise return an existing native session.
/// # Arguments
/// * `id` — Session ID to resume or import.
/// # Returns
/// The resumed native session.
/// # Errors
/// Propagates archive discovery, parsing, storage and resume failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_codex_import::Host>() -> anyhow::Result<()> {
/// let _result = codetether_codex_import::load_or_import_session::<H>("session-id").await?;
/// # Ok(()) }
/// ```
pub async fn load_or_import_session<H: crate::Host>(id: &str) -> Result<H::Session> {
    let existing = H::resume(id).await.ok();
    if let Some(codex_home) = codex_home_dir()
        && let Some(path) = find_codex_session_path_by_id(&codex_home, id)?
    {
        return import_codex_session_path::<H>(&path).await;
    }
    if let Some(existing) = existing {
        Ok(existing)
    } else {
        import_codex_session_by_id::<H>(id).await
    }
}
