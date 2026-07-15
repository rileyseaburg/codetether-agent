//! Synchronized storage for active agent traces.

use super::LiveTraceSnapshot;
use crate::session::SessionEvent;
use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref TRACES: RwLock<HashMap<String, LiveTraceSnapshot>> = RwLock::new(HashMap::new());
}

/// Start a fresh trace for an agent turn.
///
/// Replaces any stale trace stored under `name` with the supplied `prompt`.
pub(in crate::tool::agent) fn begin(name: &str, prompt: String) {
    TRACES.write().insert(
        name.to_string(),
        LiveTraceSnapshot {
            prompt,
            ..LiveTraceSnapshot::default()
        },
    );
}

/// Record one session event in an active trace.
///
/// Unknown agent names are ignored because their turn is no longer observable.
pub(in crate::tool::agent) fn observe(name: &str, event: &SessionEvent) {
    if let Some(trace) = TRACES.write().get_mut(name) {
        super::record::apply(trace, event);
    }
}

/// Remove a trace after the durable session becomes available.
///
/// Missing agent names are accepted as an idempotent cleanup operation.
pub(in crate::tool::agent) fn clear(name: &str) {
    TRACES.write().remove(name);
}

/// Clone the current trace for display.
///
/// Returns `None` when the agent has no turn in progress.
pub(in crate::tool::agent) fn snapshot(name: &str) -> Option<LiveTraceSnapshot> {
    TRACES.read().get(name).cloned()
}
