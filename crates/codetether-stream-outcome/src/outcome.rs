//! One assembled response and its terminal transport status.
use crate::StreamStop;
use codetether_provider_core::{CompletionResponse, ContentPart};

/// Result of one drain pass: the assembled (possibly partial) response and
/// the classified stop reason.
/// # Examples
/// ```
/// let outcome = codetether_stream_outcome::DrainOutcome { response: None, completed: vec![], stop: codetether_stream_outcome::StreamStop::Clean };
/// assert!(!outcome.committed());
/// ```
pub struct DrainOutcome {
    /// Assembled response, possibly partial.
    pub response: Option<CompletionResponse>,
    /// Provider-confirmed completed content items.
    pub completed: Vec<ContentPart>,
    /// Terminal transport or provider condition.
    pub stop: StreamStop,
}

impl DrainOutcome {
    /// Whether usable assistant content was committed this pass.
    pub fn committed(&self) -> bool {
        self.response
            .as_ref()
            .is_some_and(|r| !r.message.content.is_empty())
    }
}
