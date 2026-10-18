//! Durability classification for session events.

use crate::SessionEvent;

impl<S> SessionEvent<S> {
    /// Returns `true` when the event must reach the durable sink.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_session_events::{SessionEvent, TokenDelta, TokenSource};
    ///
    /// let delta = TokenDelta {
    ///     source: TokenSource::Root,
    ///     model: "m".into(),
    ///     prompt_tokens: 1,
    ///     completion_tokens: 1,
    ///     duration_ms: 0,
    /// };
    /// assert!(SessionEvent::<()>::TokenUsage(delta).is_durable());
    /// assert!(!SessionEvent::<()>::Thinking.is_durable());
    /// ```
    pub fn is_durable(&self) -> bool {
        matches!(
            self,
            Self::TokenUsage(_)
                | Self::ToolCallStart { .. }
                | Self::ToolCallComplete { .. }
                | Self::ToolCallMetadata { .. }
                | Self::ApprovalRequest(_)
                | Self::TextComplete(_)
                | Self::ThinkingComplete(_)
                | Self::UsageReport { .. }
                | Self::Done
                | Self::Error(_)
                | Self::RlmComplete(_)
                | Self::CompactionStarted(_)
                | Self::CompactionCompleted(_)
                | Self::CompactionFailed(_)
                | Self::ContextTruncated(_)
                | Self::RlmSubcallFallback(_)
        )
    }
}
