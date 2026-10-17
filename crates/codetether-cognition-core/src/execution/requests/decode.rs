//! Decode ordered tool requests from a model response.
use super::{ExtractedToolRequest, ToolRequestResponse};

/// Parse tool requests, returning an empty list for malformed output.
///
/// # Arguments
/// * `output_text` - Raw model response to decode.
///
/// # Returns
/// Requests in response order, or an empty vector on malformed JSON.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example(output_text: &str) {
/// let _result = codetether_cognition_core::execution::requests::parse_tool_requests(output_text);
/// # }
/// ```
pub fn parse_tool_requests(output_text: &str) -> Vec<ExtractedToolRequest> {
    let text = output_text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: ToolRequestResponse = match serde_json::from_str(text) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };

    parsed.tool_requests
}
