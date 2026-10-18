//! Relative paths for per-session evidence memory writebacks.
use std::path::PathBuf;

/// Return the relative evidence-writeback path for a session.
/// # Arguments
/// `session_id` identifies the writeback.
/// # Returns
/// A path beneath `.codetether/memory-writeback`.
/// # Examples
/// ```rust
/// assert!(codetether_evidence_ledger::writeback_path::for_session("s").ends_with("s.json"));
/// ```
pub fn for_session(session_id: &str) -> PathBuf {
    PathBuf::from(".codetether")
        .join("memory-writeback")
        .join(format!("{session_id}.json"))
}
