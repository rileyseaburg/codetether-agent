//! Best-effort evidence writeback respecting the existing environment switch.

/// Write nonempty evidence unless memory writeback is disabled.
/// # Arguments
/// `session_id` selects the file; `ledger` supplies evidence records.
/// # Returns
/// Returns after the best-effort write; filesystem/serialization failures are ignored.
/// # Examples
/// ```no_run
/// let ledger = codetether_evidence_ledger::ledger::build("s", &[]);
/// codetether_evidence_ledger::writeback_persist::save("s", &ledger);
/// ```
pub fn save(session_id: &str, ledger: &super::ledger::ScopeLedger) {
    if ledger.evidence.is_empty() || disabled() {
        return;
    }
    let path = super::writeback_path::for_session(session_id);
    let Some(parent) = path.parent() else { return };
    if std::fs::create_dir_all(parent).is_err() {
        return;
    }
    let Ok(body) = serde_json::to_string_pretty(&ledger.evidence) else {
        return;
    };
    let _ = std::fs::write(path, body);
}

fn disabled() -> bool {
    matches!(
        std::env::var("CODETETHER_MEMORY_WRITEBACK").ok().as_deref(),
        Some("off")
    )
}
