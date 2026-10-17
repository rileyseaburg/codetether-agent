//! Parsed tool-call contract from FunctionGemma output.

/// A single parsed tool call from FunctionGemma output.
///
/// # Examples
/// ```
/// use codetether_cognition_core::tool_routing::ParsedToolCall;
/// let call = ParsedToolCall { name: "read".into(), arguments: "{}".into() };
/// assert_eq!(call.arguments, "{}");
/// ```
#[derive(Debug, Clone)]
pub struct ParsedToolCall {
    /// Requested tool name.
    pub name: String,
    /// Arguments as a JSON object string.
    pub arguments: String,
}
