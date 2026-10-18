//! Deleted-session guard against background index resurrection.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

/// Mark a deleted session to reject late background writes.
/// # Arguments
/// Receives the session ID.
/// # Returns
/// Updates the process-local tombstone set when its lock is available.
/// # Examples
/// ```
/// codetether_recall_store::tombstone::mark("deleted-session");
/// ```
pub fn mark(session_id: &str) {
    if let Ok(mut ids) = ids().lock() {
        ids.insert(session_id.to_string());
    }
}

/// Clear the deletion guard when indexing resumes.
/// # Arguments
/// Receives the session ID.
/// # Returns
/// Removes the process-local tombstone when its lock is available.
/// # Examples
/// ```
/// codetether_recall_store::tombstone::restore("active-session");
/// ```
pub fn restore(session_id: &str) {
    if let Ok(mut ids) = ids().lock() {
        ids.remove(session_id);
    }
}

/// Check whether a session has been marked deleted.
/// # Arguments
/// Receives the session ID.
/// # Returns
/// True when marked; false when absent or the lock is poisoned.
/// # Examples
/// ```
/// let deleted = codetether_recall_store::tombstone::contains("session-id");
/// ```
pub fn contains(session_id: &str) -> bool {
    ids().lock().is_ok_and(|ids| ids.contains(session_id))
}

fn ids() -> &'static Mutex<HashSet<String>> {
    static IDS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    IDS.get_or_init(|| Mutex::new(HashSet::new()))
}
