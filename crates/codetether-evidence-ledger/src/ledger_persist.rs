//! Best-effort JSON persistence for scope ledgers.
use super::ledger::ScopeLedger;

/// Write a ledger as pretty JSON, preserving best-effort failure handling.
/// # Arguments
/// `ledger` supplies the session ID and content.
/// # Returns
/// Returns after attempting directory creation, serialization and writing; failures are ignored.
/// # Examples
/// ```no_run
/// let ledger = codetether_evidence_ledger::ledger::build("s", &[]);
/// codetether_evidence_ledger::ledger_persist::save(&ledger);
/// ```
pub fn save(ledger: &ScopeLedger) {
    let path = super::ledger_path::for_session(&ledger.session_id);
    let Some(parent) = path.parent() else { return };
    if std::fs::create_dir_all(parent).is_err() {
        return;
    }
    let Ok(body) = serde_json::to_string_pretty(ledger) else {
        return;
    };
    let _ = std::fs::write(path, body);
}
