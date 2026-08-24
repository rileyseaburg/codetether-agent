//! Removal of caller-spoofable runtime authority from raw MCP arguments.

use serde_json::Value;

pub(super) fn untrusted(mut args: Value) -> Value {
    if let Some(map) = args.as_object_mut() {
        map.retain(|key, _| !key.starts_with("__ct_"));
    }
    args
}

#[cfg(test)]
#[test]
fn strips_session_and_workspace_authority() {
    let input = serde_json::json!({"approval_id": "kept", "__ct_session_id": "spoofed"});
    assert_eq!(untrusted(input), serde_json::json!({"approval_id": "kept"}));
}
