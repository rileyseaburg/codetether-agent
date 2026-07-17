//! Append an identity to the on-disk intro ledger atomically.
//!
//! Best-effort: failure logs and returns silently so the discovery
//! loop is never blocked by a ledger write — but it never overwrites
//! a ledger it cannot read.

use super::ledger_load::load;
use super::ledger_path::ledger_path;

/// Record `identity` as introduced.
pub fn record(identity: &str) {
    let Some(path) = ledger_path() else {
        return;
    };
    let mut set = match load() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(
                error = %e,
                "Failed to load intro ledger, aborting record to prevent data loss"
            );
            return;
        }
    };
    if !set.insert(identity.trim_end_matches('/').to_string()) {
        return;
    }
    super::ledger_atomic::write_atomic(&path, &set);
}
