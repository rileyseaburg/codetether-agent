//! One-time overlay tracking for children restored directly from disk.

use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

static IDS: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// Record that a child needs its one-time resume overlay.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// # Returns
/// Unit after updating the marker set.
/// # Examples
/// ```
/// codetether_collaboration_state::residency::stale::mark("docs-stale");
/// ```
pub fn mark(agent_id: &str) {
    lock().insert(agent_id.into());
}

/// Consume a child resume-overlay marker if present.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// # Returns
/// True if a marker was present and removed.
/// # Examples
/// ```
/// let _ = codetether_collaboration_state::residency::stale::take("docs-stale");
/// ```
pub fn take(agent_id: &str) -> bool {
    lock().remove(agent_id)
}

pub(super) fn forget(agent_id: &str) {
    lock().remove(agent_id);
}

fn lock() -> std::sync::MutexGuard<'static, HashSet<String>> {
    IDS.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
