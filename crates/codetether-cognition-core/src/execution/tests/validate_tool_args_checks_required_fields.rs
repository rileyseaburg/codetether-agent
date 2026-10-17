//! validate tool args checks required fields.
use super::*;
#[test]
fn validate_tool_args_checks_required_fields() {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["command"],
        "properties": {
            "command": { "type": "string" }
        }
    });

    // Missing required field
    let args = serde_json::json!({});
    assert!(validate_tool_args(&schema, &args).is_err());

    // Has required field
    let args = serde_json::json!({ "command": "ls" });
    assert!(validate_tool_args(&schema, &args).is_ok());
}
