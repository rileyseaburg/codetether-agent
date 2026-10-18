use super::{format_structured_tool_error_preview, parse_structured_tool_error};

#[test]
fn parses_structured_tool_error() {
    let raw = r#"{
  "error": {
    "code": "MISSING_FIELD",
    "tool": "swarm_execute",
    "message": "tasks is required",
    "missing_fields": ["tasks"],
    "example": {
      "tasks": [
        {"name": "Task 1", "instruction": "Do something"}
      ]
    }
  }
}"#;

    let parsed = parse_structured_tool_error(raw).expect("expected structured error");
    assert_eq!(parsed.code.as_deref(), Some("MISSING_FIELD"));
    assert_eq!(parsed.tool.as_deref(), Some("swarm_execute"));
    assert_eq!(parsed.missing_fields, vec!["tasks"]);
    assert!(parsed.example.is_some());
}

#[test]
fn formats_structured_tool_error_preview() {
    let raw = r#"{"error":{"code":"MISSING_FIELD","tool":"swarm_execute","message":"tasks is required","missing_fields":["tasks"],"example":{"tasks":[{"name":"Task 1","instruction":"Do something"}]}}}"#;
    let preview = format_structured_tool_error_preview(raw).expect("expected preview");
    assert!(preview.contains("swarm_execute • MISSING_FIELD"));
    assert!(preview.contains("Missing: tasks"));
    assert!(preview.contains("Task 1"));
}
