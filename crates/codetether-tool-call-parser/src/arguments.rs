//! Empty argument payloads represent complete no-argument calls.

/// Parse tool-call arguments, treating an empty string as `{}`.
///
/// Streaming providers (e.g. Bedrock) send no argument deltas for a tool
/// whose schema has no properties, leaving an empty string. That is a
/// complete, argument-free call, not a truncated one.
///
/// # Errors
///
/// Returns the JSON error for non-empty arguments that do not parse.
/// # Arguments
/// Receives a raw tool-argument string; blank input is accepted.
/// # Returns
/// Parsed JSON, treating blank arguments as an empty object.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_call_parser::parse_arguments("").unwrap(), serde_json::json!({}));
/// ```
pub fn parse_arguments(arguments: &str) -> serde_json::Result<serde_json::Value> {
    if arguments.trim().is_empty() {
        return Ok(serde_json::Value::Object(serde_json::Map::new()));
    }
    serde_json::from_str(arguments)
}
