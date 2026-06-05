//! Durability classification for session events.

use super::types::SessionEvent;

impl SessionEvent {
    /// Returns `true` when the event must reach the durable sink.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::session::{SessionEvent, TokenDelta, TokenSource};
    ///
    /// let delta = TokenDelta {
    ///     source: TokenSource::Root,
    ///     model: "m".into(),
    ///     prompt_tokens: 1,
    ///     completion_tokens: 1,
    ///     duration_ms: 0,
    /// };
    /// assert!(SessionEvent::TokenUsage(delta).is_durable());
    /// assert!(!SessionEvent::Thinking.is_durable());
    /// ```
    pub fn is_durable(&self) -> bool {
        matches!(
            self,
            Self::TokenUsage(_)
                | Self::RlmComplete(_)
                | Self::CompactionStarted(_)
                | Self::CompactionCompleted(_)
                | Self::CompactionFailed(_)
                | Self::ContextTruncated(_)
                | Self::RlmSubcallFallback(_)
        )
    }
}
