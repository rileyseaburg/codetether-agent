//! Atomic on-disk write for the intro ledger.
//!
//! Writes JSON to `<path>.tmp` first, then renames over the target so
//! a crash mid-write can never leave a half-truncated ledger.

use std::collections::HashSet;
use std::path::Path;

/// Serialize `set` to JSON and commit it via temp-file rename.
pub(super) fn write_atomic(path: &Path, set: &HashSet<String>) {
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::debug!(error = %e, "Failed to create intro ledger dir");
            return;
        }
    }
    let json = match serde_json::to_string(set) {
        Ok(j) => j,
        Err(e) => {
            tracing::debug!(error = %e, "Failed to serialize intro ledger");
            return;
        }
    };
    let temp_path = path.with_extension("tmp");
    if let Err(e) = std::fs::write(&temp_path, json) {
        tracing::debug!(error = %e, "Failed to write temp intro ledger");
        return;
    }
    if let Err(e) = std::fs::rename(&temp_path, path) {
        tracing::debug!(error = %e, "Failed to commit intro ledger");
    }
}
