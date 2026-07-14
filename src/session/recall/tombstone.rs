//! Deleted-session guard against background index resurrection.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

pub(super) fn mark(session_id: &str) {
    if let Ok(mut ids) = ids().lock() {
        ids.insert(session_id.to_string());
    }
}

pub(super) fn restore(session_id: &str) {
    if let Ok(mut ids) = ids().lock() {
        ids.remove(session_id);
    }
}

pub(super) fn contains(session_id: &str) -> bool {
    ids().lock().is_ok_and(|ids| ids.contains(session_id))
}

fn ids() -> &'static Mutex<HashSet<String>> {
    static IDS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    IDS.get_or_init(|| Mutex::new(HashSet::new()))
}
