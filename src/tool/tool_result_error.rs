//! Structured error construction for [`ToolResult`].
//!
//! Split from [`super`] so the tool registry module stays within the file
//! budget. Builds an LLM-friendly JSON error payload with a machine-readable
//! code, tool name, optional missing fields, and an example invocation.

use std::collections::HashMap;

use serde_json::Value;

use super::ToolResult;

impl ToolResult {
    /// Create a structured error with code, tool name, missing fields, and example.
    ///
    /// This helps LLMs self-correct by providing actionable information about
    /// what went wrong.
    pub fn structured_error(
        code: &str,
        tool: &str,
        message: &str,
        missing_fields: Option<Vec<&str>>,
        example: Option<Value>,
    ) -> Self {
        let mut error_obj = serde_json::json!({
            "code": code,
            "tool": tool,
            "message": message,
        });
        if let Some(fields) = missing_fields {
            error_obj["missing_fields"] = serde_json::json!(fields);
        }
        if let Some(ex) = example {
            error_obj["example"] = ex;
        }
        let output = serde_json::to_string_pretty(&serde_json::json!({ "error": error_obj }))
            .unwrap_or_else(|_| format!("Error: {message}"));
        let mut metadata = HashMap::new();
        metadata.insert("error_code".to_string(), serde_json::json!(code));
        metadata.insert("tool".to_string(), serde_json::json!(tool));
        Self {
            output,
            success: false,
            metadata,
        }
    }
}
