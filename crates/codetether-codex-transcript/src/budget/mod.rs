//! FIFO retention for oversized Codex transcripts.
use codetether_provider_core::Message;
use std::collections::VecDeque;
mod finish;
pub(crate) const MAX_IMPORTED_MESSAGES: usize = 4000;

/// A capped, FIFO-eviction buffer of imported messages.
pub(crate) struct BoundedMessages {
    inner: VecDeque<Message>,
    dropped: usize,
}
impl BoundedMessages {
    pub(crate) fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            dropped: 0,
        }
    }

    /// Push a message, evicting the oldest when at capacity.
    pub(crate) fn push(&mut self, message: Message) {
        if self.inner.len() >= MAX_IMPORTED_MESSAGES {
            self.inner.pop_front();
            self.dropped = self.dropped.saturating_add(1);
        }
        self.inner.push_back(message);
    }

    /// Number of messages dropped to stay within the cap.
    #[cfg(test)]
    pub(crate) fn dropped(&self) -> usize {
        self.dropped
    }
}
#[cfg(test)]
mod tests;
