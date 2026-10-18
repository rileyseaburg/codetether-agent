//! Application path adapter for the shared introduction ledger.
pub use super::ledger_path::ledger_path;
/// Load introduction identities from the configured data directory.
///
/// # Errors
/// Returns filesystem errors when the ledger cannot be read.
pub fn load() -> Result<std::collections::HashSet<String>, std::io::Error> {
    crate::intro::ledger::load(ledger_path())
}
/// Record an introduction using the configured data directory.
pub fn record(identity: &str) {
    crate::intro::ledger::record(identity, ledger_path)
}
/// Whether `identity` was already introduced (normalized, no trailing `/`).
pub fn contains(identity: &str) -> bool {
    crate::intro::ledger::contains(identity, ledger_path())
}
