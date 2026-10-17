//! Results returned to an RLM executor after tool dispatch.

/// Result of dispatching an RLM tool call.
///
/// Variants distinguish ordinary output from a final answer that ends the loop.
/// # Examples
/// ```
/// use codetether_rlm::tools::RlmToolResult;
/// let text = match RlmToolResult::Final("done".into()) {
///     RlmToolResult::Output(text) | RlmToolResult::Final(text) => text,
/// };
/// assert_eq!(text, "done");
/// ```
pub enum RlmToolResult {
    /// Normal output to feed back to the LLM.
    Output(String),
    /// The final answer — terminates the RLM loop.
    Final(String),
}
