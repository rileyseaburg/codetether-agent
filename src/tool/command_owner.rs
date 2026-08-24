//! Stable ownership identity for persistent command sessions.

use serde_json::Value;

pub(crate) fn from_args(args: &Value) -> Option<String> {
    ["__ct_session_id", "__ct_lease_owner"]
        .into_iter()
        .find_map(|key| {
            args.get(key)
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .map(str::to_string)
}