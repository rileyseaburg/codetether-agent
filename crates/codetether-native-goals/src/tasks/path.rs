//! Resolve the on-disk path for a session's task log.

use anyhow::Result;
use std::path::PathBuf;

/// Returns `<sessions_dir>/<id>.tasks.jsonl`.
///
/// Mirrors [`codetether_session_store::lifecycle::session_path`] but with the
/// `.tasks.jsonl` suffix so the log lives alongside its session file.
pub fn task_log_path(session_id: &str) -> Result<PathBuf> {
    Ok(
        codetether_session_store::lifecycle::session_path(session_id)?
            .with_extension("tasks.jsonl"),
    )
}
