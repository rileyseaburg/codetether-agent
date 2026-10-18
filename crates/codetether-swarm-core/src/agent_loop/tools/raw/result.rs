//! Outcome of one raw tool invocation.

use codetether_provider_core::ContentPart;
use codetether_tool_core::ToolResult;

/// Unrendered tool output, image attachments, and execution outcome.
/// # Examples
/// ```
/// let result = codetether_swarm_core::agent_loop::tools::raw::RawResult::failure("failed".into(), false);
/// assert!(!result.success);
/// assert!(!result.timed_out);
/// ```
pub struct RawResult {
    /// Output text, including the existing error prefix for unsuccessful tools.
    pub output: String,
    /// Provider content parts decoded from tool-result image metadata.
    pub images: Vec<ContentPart>,
    /// Whether the underlying tool reported success.
    pub success: bool,
    /// Whether the raw invocation exceeded the agent deadline.
    pub timed_out: bool,
}

impl RawResult {
    /// Preserve a tool result's images and classify its output text.
    /// # Arguments
    /// * `result` — Completed tool output and metadata.
    /// # Returns
    /// A non-timeout result with the original success flag and decoded images.
    /// # Examples
    /// ```
    /// let result = codetether_swarm_core::agent_loop::tools::raw::RawResult::from_tool(
    ///     codetether_tool_core::ToolResult::success("ok"),
    /// );
    /// assert_eq!(result.output, "ok");
    /// ```
    pub fn from_tool(result: ToolResult) -> Self {
        Self {
            images: codetether_tool_core::images::content(Some(&result.metadata)),
            output: if result.success {
                result.output
            } else {
                format!("Tool error: {}", result.output)
            },
            success: result.success,
            timed_out: false,
        }
    }

    /// Construct a failure without image attachments.
    /// # Arguments
    /// * `output` — Failure description.
    /// * `timed_out` — Whether the failure came from the deadline.
    /// # Returns
    /// An unsuccessful result with the supplied timeout classification.
    /// # Examples
    /// ```
    /// let result = codetether_swarm_core::agent_loop::tools::raw::RawResult::failure("timeout".into(), true);
    /// assert!(result.timed_out);
    /// ```
    pub fn failure(output: String, timed_out: bool) -> Self {
        Self {
            output,
            images: Vec::new(),
            success: false,
            timed_out,
        }
    }
}
