//! Serializable text, status, and metadata returned by a tool.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Result from tool execution
/// Text, outcome, and tool-owned structured metadata.
///
/// # Examples
/// ```
/// use codetether_tool_core::ToolResult;
/// let result = ToolResult::success("done");
/// assert!(result.success);
/// assert!(result.metadata.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Text produced for the tool caller.
    pub output: String,
    /// Whether the operation reported success.
    pub success: bool,
    /// Optional structured values retained separately from the text output.
    #[serde(default)]
    pub metadata: HashMap<String, Value>,
}
