//! Borrowed values displayed by the inspector.
/// Display-only projection of completion and tool activity.
///
/// # Examples
/// ```
/// let view = codetether_inspector_view::InspectorView::default();
/// assert_eq!(view.message_count, 0);
/// ```
#[derive(Default)]
pub struct InspectorView<'a> {
    /// Completion model label.
    pub last_completion_model: Option<&'a str>,
    /// Prompt token count.
    pub last_completion_prompt_tokens: Option<usize>,
    /// Output token count.
    pub last_completion_output_tokens: Option<usize>,
    /// Completion latency in milliseconds.
    pub last_completion_latency_ms: Option<u64>,
    /// Most recent tool name.
    pub last_tool_name: Option<&'a str>,
    /// Most recent tool latency in milliseconds.
    pub last_tool_latency_ms: Option<u64>,
    /// Most recent tool outcome.
    pub last_tool_success: Option<bool>,
    /// Number of displayed messages.
    pub message_count: usize,
    /// Whether a request is running.
    pub processing: bool,
}
