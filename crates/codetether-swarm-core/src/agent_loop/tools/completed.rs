//! Completed call data consumed by the sub-agent batch recorder.
use codetether_provider_core::ContentPart;

/// A rendered tool result and optional image attachments.
/// # Examples
/// ```
/// let completed = codetether_swarm_core::agent_loop::tools::Completed {
///     id: "call".into(), name: "read".into(), output: "ok".into(),
///     images: vec![], timed_out: false,
/// };
/// assert!(!completed.timed_out);
/// ```
pub struct Completed {
    /// Provider call ID linked by the tool-result message.
    pub id: String,
    /// Tool name used in completion diagnostics.
    pub name: String,
    /// Rendered output text.
    pub output: String,
    /// Image parts appended after the textual tool result.
    pub images: Vec<ContentPart>,
    /// Whether execution or result processing exceeded the deadline.
    pub timed_out: bool,
}
