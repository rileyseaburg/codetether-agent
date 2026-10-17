//! Tool schema and model-issued calls.

use serde::{Deserialize, Serialize};

/// Schema-driven tool definition passed to the model.
///
/// # Examples
/// ```
/// use codetether_rlm::{traits::ToolDefinition};
/// let tool = ToolDefinition { name:"read".into(),description:"Read a file".into(),parameters:serde_json::json!({"type":"object"}) };
/// assert_eq!(tool.name,"read");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// A tool call returned by the model.
///
/// # Examples
/// ```
/// use codetether_rlm::{traits::ToolCall};
/// let call = ToolCall { id:"one".into(),name:"read".into(),arguments:serde_json::json!({}) };
/// assert_eq!(call.id,"one");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}
