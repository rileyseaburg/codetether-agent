//! Read the on-disk intro ledger into memory.
//!
//! `load()` returns `Ok(empty)` only when the file is absent. Other I/O
//! errors propagate so the caller can refuse to clobber a valid ledger
//! on a transient failure.

use std::collections::HashSet;

use super::ledger_path::ledger_path;

/// Load the set of identities we have already introduced ourselves to.
pub fn load() -> Result<HashSet<String>, std::io::Error> {
    let Some(path) = ledger_path() else {
        return Ok(HashSet::new());
    };
    match std::fs::read_to_string(&path) {
        Ok(raw) => Ok(serde_json::from_str(&raw).unwrap_or_default()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashSet::new()),
        Err(e) => Err(e),
    }
}
