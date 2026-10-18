//! Relative paths for per-session scope ledgers.
use std::path::PathBuf;

/// Return the relative JSON ledger path for a session.
/// # Arguments
/// `session_id` identifies the ledger.
/// # Returns
/// A path beneath `.codetether/session-ledgers`.
/// # Examples
/// ```rust
/// assert!(codetether_evidence_ledger::ledger_path::for_session("s").ends_with("s.json"));
/// ```
pub fn for_session(session_id: &str) -> PathBuf {
    PathBuf::from(".codetether")
        .join("session-ledgers")
        .join(format!("{session_id}.json"))
}
