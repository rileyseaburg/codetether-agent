//! Structured fields accompanying model-facing command output.

use serde_json::{Value, json};
use std::collections::HashMap;

use super::super::{Poll, SpawnMetadata};

pub(super) fn build(
    poll: &Poll,
    command: &SpawnMetadata,
    id: Option<u64>,
) -> HashMap<String, Value> {
    [
        ("running", json!(poll.running)),
        ("exit_code", json!(poll.exit_code)),
        ("session_id", json!(id)),
        ("wall_time_seconds", json!(poll.elapsed.as_secs_f64())),
        ("omitted_bytes", json!(poll.omitted_bytes)),
        ("sandboxed", json!(command.sandboxed)),
        ("interactive", json!(command.interactive)),
        ("cwd", json!(command.cwd.display().to_string())),
        ("unsafe_fallbacks", json!(command.unsafe_fallbacks)),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
}
