use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredToolError {
    pub code: Option<String>,
    pub tool: Option<String>,
    pub message: Option<String>,
    pub missing_fields: Vec<String>,
    pub example: Option<String>,
}

pub fn parse_structured_tool_error(text: &str) -> Option<StructuredToolError> {
    let value: Value = serde_json::from_str(text).ok()?;
    let error = value.get("error")?;

    let code = error
        .get("code")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let tool = error
        .get("tool")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let message = error
        .get("message")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let missing_fields = error
        .get("missing_fields")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let example = error
        .get("example")
        .and_then(|value| serde_json::to_string_pretty(value).ok());

    Some(StructuredToolError {
        code,
        tool,
        message,
        missing_fields,
        example,
    })
}

pub fn format_structured_tool_error_preview(text: &str) -> Option<String> {
    let parsed = parse_structured_tool_error(text)?;
    let mut lines = Vec::new();

    let mut header_bits = Vec::new();
    if let Some(tool) = parsed.tool.as_deref() {
        header_bits.push(tool.to_string());
    }
    if let Some(code) = parsed.code.as_deref() {
        header_bits.push(code.to_string());
    }
    if !header_bits.is_empty() {
        lines.push(header_bits.join(" • "));
    }

    if let Some(message) = parsed.message.as_deref() {
        lines.push(message.to_string());
    }

    if !parsed.missing_fields.is_empty() {
        lines.push(format!("Missing: {}", parsed.missing_fields.join(", ")));
    }

    if let Some(example) = parsed.example.as_deref() {
        lines.push("Example:".to_string());
        lines.extend(example.lines().map(ToOwned::to_owned));
    }

    Some(lines.join("\n"))
}

#[cfg(test)]
mod tests {
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
}
