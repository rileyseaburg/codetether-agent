//! Human-readable event summaries for live frontend activity feeds.
use crate::SessionEvent;

/// Describe an event without exposing tool arguments or provider internals.
///
/// # Arguments
/// * `event` - The runtime event received by the frontend.
/// # Returns
/// A short label suitable for a visible event feed.
/// # Examples
/// ```
/// use codetether_session_events::{activity::label, SessionEvent};
/// assert_eq!(label(&SessionEvent::<()>::Thinking), "Thinking update received");
/// ```
pub fn label<S>(event: &SessionEvent<S>) -> String {
    match event {
        SessionEvent::Thinking => "Thinking update received".into(),
        SessionEvent::ProviderKeepalive => "Waiting for model — keepalive (no new output)".into(),
        SessionEvent::ThinkingComplete(_) => "Thinking text received".into(),
        SessionEvent::TextChunk(_) => "Reply text received".into(),
        SessionEvent::TextComplete(_) => "Reply complete".into(),
        SessionEvent::ToolCallStart { name, .. } => format!("Tool started: {name}"),
        SessionEvent::ToolCallComplete { name, success, .. } => format!(
            "Tool {}: {name}",
            if *success { "completed" } else { "failed" }
        ),
        SessionEvent::ToolHeartbeat {
            name, elapsed_secs, ..
        } => format!("Tool heartbeat: {name} ({elapsed_secs}s)"),
        SessionEvent::ToolOutputChunk { name, .. } => format!("Tool output: {name}"),
        SessionEvent::ToolCallMetadata { name, .. } => format!("Tool details: {name}"),
        SessionEvent::ApprovalRequest(_) => "Waiting for approval".into(),
        SessionEvent::StreamRetry(_) => "Retrying model stream".into(),
        SessionEvent::Done => "Turn complete".into(),
        SessionEvent::Error(_) => "Error received — see conversation".into(),
        SessionEvent::SessionSync(_) => "Session synchronized".into(),
        SessionEvent::TokenEstimate(_)
        | SessionEvent::TokenUsage(_)
        | SessionEvent::UsageReport { .. } => "Token usage updated".into(),
        SessionEvent::CompactionStarted(_) => "Context compaction started".into(),
        SessionEvent::CompactionCompleted(_) => "Context compaction complete".into(),
        SessionEvent::CompactionFailed(_) => "Context compaction failed".into(),
        SessionEvent::ContextTruncated(_) => "Context truncated".into(),
        SessionEvent::RlmProgress(_) => "Context processing update".into(),
        SessionEvent::RlmComplete(_) => "Context processing complete".into(),
        SessionEvent::RlmSubcallFallback(_) => "Context model fallback".into(),
        SessionEvent::PlayBreak(_) => "Background activity updated".into(),
    }
}
