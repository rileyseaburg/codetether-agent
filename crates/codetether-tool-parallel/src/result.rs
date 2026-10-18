//! Parallel tool job output.

#[derive(Clone)]
/// Completed call before application-specific routing and message recording.
/// # Examples
/// ```
/// let output = codetether_tool_parallel::Output {
///     tool_id: "1".into(), tool_name: "read".into(), tool_input: serde_json::json!({}),
///     content: "hello".into(), success: true, images: vec![],
/// };
/// assert!(output.success);
/// ```
pub struct Output {
    /// Invocation identifier used by events and the tool-result message.
    pub tool_id: String,
    /// Normalized tool name.
    pub tool_name: String,
    /// Normalized model input, before runtime context enrichment.
    pub tool_input: serde_json::Value,
    /// Complete textual output before compaction.
    pub content: String,
    /// Tool-reported execution status.
    pub success: bool,
    /// Attachments bypass text/RLM compaction and stay with this tool call.
    pub images: Vec<codetether_provider_core::ContentPart>,
}
