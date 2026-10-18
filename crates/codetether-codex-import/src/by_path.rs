//! by path import workflow.
use crate::persist_imported_session;
use anyhow::Context;
use anyhow::Result;
use codetether_codex_discovery::{codex_home_dir_for_path, load_session_index, read_session_meta};

/// Import a rollout using its indexed title and resume the stored session.
/// # Arguments
/// * `path` — Codex rollout beneath a .codex directory.
/// # Returns
/// The resumed native session.
/// # Errors
/// Propagates archive discovery, parsing, storage and resume failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_codex_import::Host>() -> anyhow::Result<()> {
/// let _result = codetether_codex_import::import_codex_session_path::<H>(std::path::Path::new(".codex/sessions/rollout.jsonl")).await?;
/// # Ok(()) }
/// ```
pub async fn import_codex_session_path<H: crate::Host>(
    path: &std::path::Path,
) -> Result<H::Session> {
    let Some(codex_home) = codex_home_dir_for_path(path) else {
        anyhow::bail!("Could not resolve Codex home for {}", path.display());
    };
    let meta = read_session_meta(path)?
        .with_context(|| format!("Missing session metadata in {}", path.display()))?;
    let session = H::parse(
        path,
        load_session_index(&codex_home)
            .get(&meta.id)
            .map(String::as_str),
    )?;
    let _ = persist_imported_session::<H>(session, &H::sessions_dir()?).await?;
    H::resume(&meta.id).await
}
