//! Normalized provider tool-call data.
use serde_json::Value;

/// Normalized provider tool call ready for policy checks and execution.
/// # Examples
/// ```
/// let call = codetether_prompt_loop::Call::new("id".into(), "read".into(), serde_json::json!({"path":"README.md"}));
/// assert_eq!(call.id, "id");
/// ```
pub struct Call {
    /// Provider-supplied call identifier.
    pub id: String,
    /// Canonical executable tool name.
    pub name: String,
    /// Normalized JSON input passed to the tool.
    pub input: Value,
}

impl Call {
    /// Normalizes a parsed provider tool-call tuple.
    pub fn new(id: String, name: String, input: Value) -> Self {
        let (name, input) =
            codetether_edit_policy::normalize_tool_call_for_execution(&name, &input);
        Self { id, name, input }
    }
}
