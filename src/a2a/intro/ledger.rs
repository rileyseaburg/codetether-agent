//! Persistent ledger of peers already introduced to.
//!
//! The in-memory `discovered` set in the discovery loops forgets on
//! restart, which caused thousands of duplicate intros. This ledger
//! persists introduced endpoints to `<data_dir>/a2a/introduced.json`
//! keyed by endpooint only (peer names embed PIDs and churn per restart).

use std::collections::HashSet;
use std::path::PathBuf;

/// Where the ledger lives: `<data_dir>/a2a/introduced.json`.
fn ledger_path() -> Option<PathBuf> {
    crate::config::Config::data_dir().map(|d| d.join("a2a").join("introduced.json"))
}

/// Load the set of endpoints we have already introduced ourselves to.
pub fn load() -> HashSet<String> {
    let Some(path) = ledger_path() else {
        return HashSet::new();
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return HashSet::new();
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

/// Record `endpoint` as introduced. Best-effort; failures only log.
pub fn record(endpoint: &str) {
    let Some(path) = ledger_path() else {
        return;
    };
    let mut set = load();
    if !set.insert(endpoint.trim_end_matches('/').to_string()) {
        return;
    }
    if let Some(parent) = path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        tracing::debug!(error = %e, "Failed to create intro ledger dir");
        return;
    }
    match serde_json::to_string(&set) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                tracing::debug!(error = %e, "Failed to persist intro ledger");
            }
        }
        Err(e) => tracing::debug!(error = %e, "Failed to serialize intro ledger"),
    }
}

/// Whether `endpoint` was already introduced to (normalized, no trailing `/`).
pub fn contains(endpoint: &str) -> bool {
    load().contains(endpoint.trim_end_matches('/'))
}
