use super::extract;
use crate::provider::ToolDefinition;
use serde_json::json;

fn tool() -> ToolDefinition {
    ToolDefinition {
        name: "exec_command".into(),
        description: String::new(),
        parameters: json!({
            "type": "object",
            "properties": {"cmd": {"type": "string"}},
            "required": ["cmd"],
            "additionalProperties": false
        }),
    }
}

fn markup(name: &str, arguments: &str) -> String {
    format!(r#"<tool_call>{{"name":"{name}","arguments":{arguments}}}</tool_call>"#)
}

#[test]
fn accepts_advertised_schema_conforming_call() {
    let (_, calls) = extract(&markup("exec_command", r#"{"cmd":"pwd"}"#), &[tool()], &[]).unwrap();
    assert_eq!(calls.len(), 1);
}

#[test]
fn rejects_unadvertised_tool_name() {
    let error = extract(&markup("delete_world", "{}"), &[tool()], &[]).unwrap_err();
    assert!(error.to_string().contains("not advertised"));
}

#[test]
fn rejects_missing_required_argument() {
    let error = extract(&markup("exec_command", "{}"), &[tool()], &[]).unwrap_err();
    assert!(format!("{error:#}").contains("missing required field `cmd`"));
}

#[test]
fn rejects_wrong_argument_type() {
    let error = extract(&markup("exec_command", r#"{"cmd":9}"#), &[tool()], &[]).unwrap_err();
    assert!(format!("{error:#}").contains("expected string"));
}
