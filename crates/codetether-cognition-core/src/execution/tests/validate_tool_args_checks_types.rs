//! validate tool args checks types.
use super::*;
#[test]
fn validate_tool_args_checks_types() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "command": { "type": "string" },
            "timeout": { "type": "number" }
        }
    });

    // Wrong type
    let args = serde_json::json!({ "command": 42 });
    assert!(validate_tool_args(&schema, &args).is_err());

    // Correct types
    let args = serde_json::json!({ "command": "ls", "timeout": 30 });
    assert!(validate_tool_args(&schema, &args).is_ok());
}
