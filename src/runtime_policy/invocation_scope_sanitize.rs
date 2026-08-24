//! Top-level removal of trusted runtime transport metadata.

use serde_json::Value;

const VOLATILE: &[&str] = &["approval_id", "_tool_call_id", "__ct_lease_owner"];

pub(super) fn value(input: &Value) -> Value {
    let mut scoped = input.clone();
    if let Some(map) = scoped.as_object_mut() {
        map.retain(|key, _| !VOLATILE.contains(&key.as_str()));
    }
    scoped
}
