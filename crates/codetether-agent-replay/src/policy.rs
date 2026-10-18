//! Prior-context and read-only policy for replay batches.
use crate::*;
use codetether_tool_core::readonly::is_read_only;
/// Return a structured denial when a call violates session access policy.
/// Check whether a serialized tool call violates the session policy.
/// # Arguments
/// Receives the current session and the call or batch to evaluate.
/// # Returns
/// The native denial result, or None when no denial applies.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_replay::*;
/// # fn example<H: Host>(session: &H::Session, call: codetether_provider_core::ContentPart) {
/// let _ = blocked::<H>(session, &call);
/// # }
/// ```
pub fn blocked<H: Host>(session: &H::Session, part: &ContentPart) -> Option<ToolResult> {
    let call = part.as_tool_call()?;
    H::blocked(session, call.name, call.arguments)
}

/// Allow parallel dispatch only when every call is read-only and permitted.
/// Require multiple read-only calls that all pass the session policy.
/// # Arguments
/// Receives the current session and the call or batch to evaluate.
/// # Returns
/// True only when the whole batch is eligible for parallel execution.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_replay::*;
/// # fn example<H: Host>(session: &H::Session, call: codetether_provider_core::ContentPart) {
/// assert!(!can_parallelize::<H>(session, &[]));
/// # }
/// ```
pub fn can_parallelize<H: Host>(session: &H::Session, calls: &[ContentPart]) -> bool {
    calls.len() > 1
        && calls.iter().all(|part| {
            part.as_tool_call().is_some_and(|call| {
                is_read_only(call.name) && blocked::<H>(session, part).is_none()
            })
        })
}
