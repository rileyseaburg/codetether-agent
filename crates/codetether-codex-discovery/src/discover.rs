use super::home::codex_home_dir;
use super::index::load_session_index;
use super::info::CodexSessionInfo;
use super::paths::{canonicalize_loose, is_jsonl};
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

/// Discover matching sessions under the configured Codex home.
/// # Arguments
/// * `dir` — Workspace directory to match.
/// # Returns
/// Matching sessions sorted newest first.
/// # Errors
/// Propagates discovery failures; invalid candidate files are logged and skipped.
/// # Examples
/// ```no_run
/// let result = codetether_codex_discovery::discover_codex_sessions_for_directory(std::path::Path::new("."));
/// assert!(result.is_ok());
/// ```
pub fn discover_codex_sessions_for_directory(dir: &Path) -> Result<Vec<CodexSessionInfo>> {
    discover_codex_sessions_in(codex_home_dir().as_deref(), dir)
}

/// Discover matching sessions under an explicit optional Codex home.
/// # Arguments
/// * `codex_home` — Archive home, or None for an empty result.
/// * `dir` — Workspace directory to match.
/// # Returns
/// Matching sessions sorted newest first.
/// # Errors
/// Propagates discovery failures; invalid candidate files are logged and skipped.
/// # Examples
/// ```no_run
/// let result = codetether_codex_discovery::discover_codex_sessions_in(None, std::path::Path::new("."));
/// assert!(result.is_ok());
/// ```
pub fn discover_codex_sessions_in(
    codex_home: Option<&Path>,
    dir: &Path,
) -> Result<Vec<CodexSessionInfo>> {
    let Some(codex_home) = codex_home else {
        return Ok(Vec::new());
    };
    let sessions_root = codex_home.join("sessions");
    if !sessions_root.exists() {
        return Ok(Vec::new());
    }

    let titles = load_session_index(codex_home);
    let canonical_dir = canonicalize_loose(dir);
    let mut sessions = Vec::new();

    for entry in WalkDir::new(&sessions_root)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();
        if !is_jsonl(path) {
            continue;
        }
        if let Some(session) = super::candidate::summarize_matching(path, &canonical_dir, &titles) {
            sessions.push(session);
        }
    }

    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}
