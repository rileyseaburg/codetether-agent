use serde::{Deserialize, Serialize};

/// Schema-driven tool definition passed to the model.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::ToolDefinition;
/// let tool = ToolDefinition {
///     name: "shell".into(),
///     description: "Run a shell command".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// };
/// assert_eq!(tool.name, "shell");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool function name.
    pub name: String,
    /// Human-readable description used to decide when to call.
    pub description: String,
    /// JSON Schema for the tool's input parameters.
    pub parameters: serde_json::Value,
}
