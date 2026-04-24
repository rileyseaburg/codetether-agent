//! Resolve the on-disk path for a session's task log.

use anyhow::Result;
use std::path::PathBuf;

/// Returns `<sessions_dir>/<id>.tasks.jsonl`.
///
/// Mirrors [`crate::session::Session::session_path`] but with the
/// `.tasks.jsonl` suffix so the log lives alongside its session file.
pub fn task_log_path(session_id: &str) -> Result<PathBuf> {
    let dir = crate::config::Config::data_dir()
        .map(|d| d.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
    Ok(dir.join(format!("{session_id}.tasks.jsonl")))
}
