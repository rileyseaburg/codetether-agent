//! Append an identity to the on-disk intro ledger atomically.
//!
//! Best-effort: failure logs and returns silently so the discovery
//! loop is never blocked by a ledger write — but it never overwrites
//! a ledger it cannot read.

use super::load;
use std::path::PathBuf;

/// Record `identity` as introduced.
pub fn record(identity: &str, mut resolve_path: impl FnMut() -> Option<PathBuf>) {
    let Some(path) = resolve_path() else {
        return;
    };
    let mut set = match load(resolve_path()) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(target: "codetether_agent::a2a::intro::ledger::ledger_record",
                error = %e,
                "Failed to load intro ledger, aborting record to prevent data loss"
            );
            return;
        }
    };
    if !set.insert(identity.trim_end_matches('/').to_string()) {
        return;
    }
    super::atomic::write_atomic(&path, &set);
}
