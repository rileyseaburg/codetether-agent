//! Synchronized storage for active agent traces.

use super::LiveTraceSnapshot;
use crate::TraceEvent;
use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref TRACES: RwLock<HashMap<String, LiveTraceSnapshot>> = RwLock::new(HashMap::new());
}

/// Start a fresh trace for an agent turn.
/// # Arguments
/// * `name` — Canonical agent identity.
/// * `prompt` — Request text for the new turn.
/// # Returns
/// Unit after replacing any existing trace.
/// # Examples
/// See [crate] for the observation lifecycle.
///
/// Replaces any stale trace stored under `name` with the supplied `prompt`.
pub fn begin(name: &str, prompt: String) {
    TRACES.write().insert(
        name.to_string(),
        LiveTraceSnapshot {
            prompt,
            ..LiveTraceSnapshot::default()
        },
    );
}

/// Record one session event in an active trace.
/// # Arguments
/// * `name` — Canonical agent identity.
/// * `event` — Borrowed fields for this event.
/// # Returns
/// Unit after updating an existing trace, or ignoring an unknown identity.
/// # Examples
/// See [crate::TraceEvent] for constructing an observation.
///
/// Unknown agent names are ignored because their turn is no longer observable.
pub fn observe(name: &str, event: TraceEvent<'_>) {
    if let Some(trace) = TRACES.write().get_mut(name) {
        super::record::apply(trace, event);
    }
}

/// Remove a trace after the durable session becomes available.
/// # Arguments
/// * `name` — Canonical agent identity.
/// # Returns
/// Unit after idempotent removal.
/// # Examples
/// See [crate] for the observation lifecycle.
///
/// Missing agent names are accepted as an idempotent cleanup operation.
pub fn clear(name: &str) {
    TRACES.write().remove(name);
}

/// Clone the current trace for display.
/// # Arguments
/// * `name` — Canonical agent identity.
/// # Returns
/// An owned snapshot, or none when no trace exists.
/// # Examples
/// See [crate::LiveTraceSnapshot] for the returned fields.
///
/// Returns `None` when the agent has no turn in progress.
pub fn snapshot(name: &str) -> Option<LiveTraceSnapshot> {
    TRACES.read().get(name).cloned()
}
