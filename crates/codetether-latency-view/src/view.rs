//! Inputs projected from the application's current UI state.
use codetether_telemetry::chat_latency::ChatLatencyStats;
/// State required to render the latency inspector.
/// # Examples
/// ```
/// use codetether_latency_view::LatencyView;
/// let stats = codetether_telemetry::chat_latency::ChatLatencyStats::default();
/// let view = LatencyView {
///     cwd_display: ".".into(), processing: false, current_request_elapsed_ms: None,
///     current_request_first_token_ms: None, last_request_first_token_ms: None,
///     last_request_last_token_ms: None, last_completion_latency_ms: None,
///     last_completion_model: None, last_completion_prompt_tokens: None,
///     last_completion_output_tokens: None, last_tool_latency_ms: None,
///     last_tool_name: None, last_tool_success: None, status: "idle".into(),
///     chat_latency: &stats, tools: Vec::new(),
/// };
/// assert!(!view.processing);
/// ```
pub struct LatencyView<'a> {
    /// Display name of the current workspace.
    pub cwd_display: String,
    /// Whether a request is in flight.
    pub processing: bool,
    /// Elapsed duration of the in-flight request.
    pub current_request_elapsed_ms: Option<u64>,
    /// First-token duration of the in-flight request.
    pub current_request_first_token_ms: Option<u64>,
    /// First-token duration of the previous request.
    pub last_request_first_token_ms: Option<u64>,
    /// Last-token duration of the previous request.
    pub last_request_last_token_ms: Option<u64>,
    /// Most recent provider round-trip duration.
    pub last_completion_latency_ms: Option<u64>,
    /// Model used for the most recent provider completion.
    pub last_completion_model: Option<String>,
    /// Input tokens in the most recent completion.
    pub last_completion_prompt_tokens: Option<usize>,
    /// Output tokens in the most recent completion.
    pub last_completion_output_tokens: Option<usize>,
    /// Duration of the last timed tool.
    pub last_tool_latency_ms: Option<u64>,
    /// Name of the last timed tool.
    pub last_tool_name: Option<String>,
    /// Whether the last timed tool succeeded.
    pub last_tool_success: Option<bool>,
    /// Current UI status text.
    pub status: String,
    /// Shared rolling statistics for completed turns.
    pub chat_latency: &'a ChatLatencyStats,
    /// Timed tool records in chronological order.
    pub tools: Vec<crate::ToolLatency>,
}
/// One completed tool record with a measured duration.
/// # Examples
/// ```
/// let row = codetether_latency_view::ToolLatency { name: "read".into(), success: true, duration_ms: 5 };
/// assert_eq!(row.duration_ms, 5);
/// ```
pub struct ToolLatency {
    /// Tool name.
    pub name: String,
    /// Whether the tool succeeded.
    pub success: bool,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u64,
}
