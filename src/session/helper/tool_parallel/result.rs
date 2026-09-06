//! Parallel tool job output.

#[derive(Clone)]
pub(crate) struct Output {
    pub tool_id: String,
    pub tool_name: String,
    pub tool_input: serde_json::Value,
    pub content: String,
    pub success: bool,
    /// Attachments bypass text/RLM compaction and stay with this tool call.
    pub images: Vec<crate::provider::ContentPart>,
}
