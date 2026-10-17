//! Application path adapter for the shared introduction ledger.
#[path = "ledger_path.rs"]
mod ledger_path;
/// Load introduction identities from the configured data directory.
///
/// # Errors
/// Returns filesystem errors when the ledger cannot be read.
pub fn load() -> Result<std::collections::HashSet<String>, std::io::Error> {
    codetether_a2a_http::intro::ledger::load(ledger_path::ledger_path())
}
/// Record an introduction using the configured data directory.
pub fn record(identity: &str) {
    codetether_a2a_http::intro::ledger::record(identity, ledger_path::ledger_path)
}
/// Whether `identity` was already introduced (normalized, no trailing `/`).
pub fn contains(identity: &str) -> bool {
    load()
        .unwrap_or_default()
        .contains(identity.trim_end_matches('/'))
}
