use super::ledger::ScopeLedger;

pub(crate) fn save(ledger: &ScopeLedger) {
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
