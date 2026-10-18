//! Rendered tool output and post-execution classifications.
use serde_json::Value;
use std::collections::HashMap;

/// Rendered result and classifications produced by one tool execution.
/// # Examples
/// ```
/// let result = codetether_tool_outcome::Outcome {
///     rendered: "done".into(), success: true, metadata: None,
///     duration_ms: 1, codesearch_miss: false,
/// };
/// assert!(result.success);
/// ```
pub struct Outcome {
    /// Output presented to the model and streaming caller.
    pub rendered: String,
    /// Whether the tool reported success.
    pub success: bool,
    /// Structured metadata returned by the tool.
    pub metadata: Option<HashMap<String, Value>>,
    /// End-to-end execution duration in milliseconds.
    pub duration_ms: u64,
    /// Whether a code-search tool returned no matches.
    pub codesearch_miss: bool,
}
