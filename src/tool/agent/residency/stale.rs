//! One-time overlay tracking for children restored directly from disk.

use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

static IDS: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub(in crate::tool::agent) fn mark(agent_id: &str) {
    lock().insert(agent_id.into());
}

pub(super) fn take(agent_id: &str) -> bool {
    lock().remove(agent_id)
}

pub(super) fn forget(agent_id: &str) {
    lock().remove(agent_id);
}

fn lock() -> std::sync::MutexGuard<'static, HashSet<String>> {
    IDS.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
