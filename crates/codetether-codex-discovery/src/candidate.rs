//! Summarize one matching archive, logging and skipping invalid candidates.
use crate::{
    info::CodexSessionInfo, meta::read_session_meta, paths::canonicalize_loose,
    summary::summarize_codex_session_from_path,
};
use std::{collections::HashMap, path::Path};
pub(crate) fn summarize_matching(
    path: &Path,
    canonical_dir: &Path,
    titles: &HashMap<String, String>,
) -> Option<CodexSessionInfo> {
    let meta = match read_session_meta(path) {
        Ok(Some(m)) => m,
        Ok(None) => return None,
        Err(err) => {
            tracing::warn!(target: "codetether_agent::session::codex_import::discover", path = %path.display(), error = %err, "Skipping unreadable Codex session meta");
            return None;
        }
    };
    if canonicalize_loose(Path::new(&meta.cwd)).as_path() != canonical_dir {
        return None;
    }
    match summarize_codex_session_from_path(path, titles.get(&meta.id).map(String::as_str)) {
        Ok(session) => Some(session),
        Err(error) => {
            tracing::warn!(target: "codetether_agent::session::codex_import::discover", path = %path.display(), error = %error, "Skipping invalid Codex session");
            None
        }
    }
}
