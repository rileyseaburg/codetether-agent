//! by id import workflow.
use crate::import_codex_session_path;
use anyhow::Result;
use codetether_codex_discovery::{codex_home_dir, find_codex_session_path_by_id};

/// Import and resume the Codex archive with the given ID.
/// # Arguments
/// * `id` — Codex session ID to locate.
/// # Returns
/// The resumed native session.
/// # Errors
/// Propagates archive discovery, parsing, storage and resume failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_codex_import::Host>() -> anyhow::Result<()> {
/// let _result = codetether_codex_import::import_codex_session_by_id::<H>("session-id").await?;
/// # Ok(()) }
/// ```
pub async fn import_codex_session_by_id<H: crate::Host>(id: &str) -> Result<H::Session> {
    let Some(codex_home) = codex_home_dir() else {
        anyhow::bail!("Codex home directory not found");
    };
    let Some(path) = find_codex_session_path_by_id(&codex_home, id)? else {
        anyhow::bail!("Codex session not found: {id}");
    };
    import_codex_session_path::<H>(&path).await
}
