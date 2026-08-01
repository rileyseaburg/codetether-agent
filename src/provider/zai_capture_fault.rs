//! Fault classification for output-free Z.AI streams.
//!
//! GLM legitimately ends a turn with no new content: after a tool result it may
//! judge the work finished and stop, reporting `finish_reason: "stop"`. Treating
//! that as an empty-stream fault made the session restart a stream the model had
//! already completed — five times — then fail the turn. Observed live as
//! "trying to resume streams after the model says its done".
//!
//! `finish_reason` is authoritative when present: a stated terminal reason means
//! the provider closed deliberately, so only a missing reason indicates
//! truncation.
//!
//! Output presence is equally authoritative. A turn whose only output was a tool
//! call is complete even when the provider omits `finish_reason`, so callers pass
//! [`OutputSeen`] rather than a bare "no text" boolean. Treating a tool-only turn
//! as empty caused a live `frames=99, finish_reason=none` failure on a turn whose
//! only work was a `bash` call.

use crate::provider::zai::stream_output::OutputSeen;

/// Returns `true` when an output-free stream should be treated as a fault.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::capture::fault::is_fault;
///
/// assert!(!is_fault(Some("stop")));        // deliberate completion
/// assert!(!is_fault(Some("tool_calls")));  // handed off to tools
/// assert!(is_fault(None));                 // ended without saying why
/// assert!(is_fault(Some("length")));       // truncated by token limit
/// ```
pub fn is_fault(finish_reason: Option<&str>) -> bool {
    !matches!(finish_reason, Some("stop") | Some("tool_calls"))
}

/// Builds the diagnostic message for a stream that produced no content.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::capture::fault::message;
///
/// let text = message(0, None);
/// assert!(text.contains("frames=0"));
/// assert!(text.contains("finish_reason=none"));
/// assert!(text.contains("CODETETHER_ZAI_CAPTURE_DIR"));
/// ```
pub fn message(frames: usize, finish_reason: Option<&str>) -> String {
    format!(
        "Z.AI stream ended without producing any content (frames={frames}, \
         finish_reason={}). Set CODETETHER_ZAI_CAPTURE_DIR to capture raw SSE.",
        finish_reason.unwrap_or("none"),
    )
}

/// Returns a terminal error message when an output-free stream is a real fault.
///
/// A stream that emitted any output — prose or tool calls — is never a fault,
/// regardless of `finish_reason`.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::capture::fault::check;
/// use codetether_agent::provider::zai::stream_output::OutputSeen;
///
/// let nothing = OutputSeen::default();
/// let tool_only = OutputSeen { text: false, tool_calls: true };
///
/// assert!(check(nothing, 3, Some("stop")).is_none()); // model stopped
/// assert!(check(tool_only, 99, None).is_none());      // tool call is output
/// assert!(check(nothing, 0, None).is_some());         // truncated
/// ```
pub fn check(seen: OutputSeen, frames: usize, finish_reason: Option<&str>) -> Option<String> {
    (seen.is_empty() && is_fault(finish_reason)).then(|| message(frames, finish_reason))
}

#[cfg(test)]
#[path = "zai_capture_fault_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "zai_capture_fault_check_tests.rs"]
mod check_tests;
