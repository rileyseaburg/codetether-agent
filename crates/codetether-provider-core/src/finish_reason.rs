use serde::{Deserialize, Serialize};

/// Why generation stopped: natural stop, length limit, tool calls, filtering or error.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::FinishReason;
/// match FinishReason::Stop {
///     FinishReason::Stop => assert_eq!(serde_json::to_string(&FinishReason::Stop).unwrap(), "\"stop\""),
///     _ => panic!("expected natural stop"),
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stop or stop sequence hit.
    Stop,
    /// Hit the `max_tokens` limit.
    Length,
    /// The model requested tool execution.
    ToolCalls,
    /// Content was filtered by safety systems.
    ContentFilter,
    /// An error occurred.
    Error,
}
