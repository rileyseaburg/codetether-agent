//! Mutable projection of the metrics cleared when a transcript is replaced.
use codetether_telemetry::{RequestTimingState, ToolTimingState};
/// Metric fields owned by the application and reset on transcript replacement.
/// Construct this from disjoint mutable fields of the application state.
///
/// # Examples
/// ```
/// use codetether_chat_transcript::sync::Metrics;
/// let mut request = codetether_telemetry::RequestTimingState::default();
/// let mut tool = codetether_telemetry::ToolTimingState::default();
/// let (mut model, mut latency, mut prompt_tokens, mut output_tokens) =
///     (Some("model".to_string()), Some(12), Some(4), Some(5));
/// Metrics { request: &mut request, tool: &mut tool, model: &mut model,
///     latency: &mut latency, prompt_tokens: &mut prompt_tokens,
///     output_tokens: &mut output_tokens }.reset();
/// assert!(model.is_none());
/// assert!(latency.is_none());
/// ```
pub struct Metrics<'a> {
    /// Current and completed request latency samples.
    pub request: &'a mut RequestTimingState,
    /// Current and completed tool timing samples.
    pub tool: &'a mut ToolTimingState,
    /// Last completion's model selection.
    pub model: &'a mut Option<String>,
    /// Last completion's elapsed time.
    pub latency: &'a mut Option<u64>,
    /// Last completion's prompt token count.
    pub prompt_tokens: &'a mut Option<usize>,
    /// Last completion's output token count.
    pub output_tokens: &'a mut Option<usize>,
}
impl Metrics<'_> {
    /// Clear completed-turn metrics while preserving pending request/tool state.
    pub fn reset(self) {
        self.request.current_request_first_token_ms = None;
        self.request.current_request_last_token_ms = None;
        self.request.last_request_first_token_ms = None;
        self.request.last_request_last_token_ms = None;
        *self.model = None;
        *self.latency = None;
        *self.prompt_tokens = None;
        *self.output_tokens = None;
        self.tool.last_tool_name = None;
        self.tool.last_tool_latency_ms = None;
        self.tool.last_tool_success = None;
        self.request.chat_latency.clear();
    }
}
