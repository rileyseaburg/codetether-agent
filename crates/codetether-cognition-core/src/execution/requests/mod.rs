//! Tool extraction request schema, prompt building and argument validation.
//!
//! Use [extraction_prompts] before model invocation, [parse_tool_requests] on
//! its response, and [validate_tool_args] before dispatching a request.
use serde::Deserialize;
use serde_json::Value;
mod decode;
mod json_type;
mod prompts;
mod validate;
pub use decode::parse_tool_requests;
pub use prompts::extraction_prompts;
pub use validate::validate_tool_args;

/// Raw tool request extracted from LLM structured output.
///
/// # Examples
/// ```
/// use codetether_cognition_core::execution::requests::ExtractedToolRequest;
/// let request = ExtractedToolRequest { tool: "read".into(), arguments: serde_json::json!({}) };
/// assert_eq!(request.tool, "read");
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct ExtractedToolRequest {
    pub tool: String,
    pub arguments: Value,
}

/// Wrapper for tool request extraction response.
#[derive(Debug, Deserialize)]
struct ToolRequestResponse {
    tool_requests: Vec<ExtractedToolRequest>,
}
