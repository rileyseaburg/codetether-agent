//! Convert discovered Codex metadata into common session summaries.
use anyhow::Result;
use codetether_codex_discovery::discover_codex_sessions_for_directory;
use codetether_session_metadata::SessionSummary;
use std::path::Path;

/// List Codex summaries with the existing title prefix.
/// # Arguments
/// * `dir` — Workspace directory whose sessions should be listed.
/// # Returns
/// Matching summaries sorted by descending update time.
/// # Errors
/// Propagates archive discovery failures.
/// # Examples
/// ```no_run
/// # fn example() -> anyhow::Result<()> {
/// let rows = codetether_session_catalog::list_codex_sessions_for_directory(std::path::Path::new("."))?;
/// assert!(rows.windows(2).all(|w| w[0].updated_at >= w[1].updated_at));
/// # Ok(()) }
/// ```
pub fn list_codex_sessions_for_directory(dir: &Path) -> Result<Vec<SessionSummary>> {
    Ok(discover_codex_sessions_for_directory(dir)?
        .into_iter()
        .map(|session| SessionSummary {
            id: session.id.clone(),
            title: Some(format!("[Codex] {}", session.title.unwrap_or(session.id))),
            created_at: session.created_at,
            updated_at: session.updated_at,
            message_count: session.message_count,
            agent: session.agent,
            directory: session.directory,
        })
        .collect())
}
