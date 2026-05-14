use crate::session::Session;

pub(crate) fn save(session: &Session, ledger: &super::ledger::ScopeLedger) {
    if ledger.evidence.is_empty() || disabled() {
        return;
    }
    let path = super::writeback_path::for_session(&session.id);
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
