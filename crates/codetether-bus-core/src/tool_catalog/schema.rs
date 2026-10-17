use super::*;

/// One tool's name, description, and JSON-Schema parameters.
///
/// # Examples
///
/// ```rust
/// use codetether_bus_core::tool_catalog::ToolSchema;
///
/// let schema = ToolSchema {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// };
/// assert_eq!(schema.name, "read");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolSchema {
    /// Tool name as the model must emit it.
    pub name: String,
    /// Human-readable purpose shown to the model.
    pub description: String,
    /// JSON Schema for the tool arguments.
    pub parameters: serde_json::Value,
}
