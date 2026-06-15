//! Memory budget for importing large Codex rollout files.
//!
//! Some Codex rollouts grow to hundreds of megabytes. Materializing every
//! message into an unbounded `Vec` (and then serializing a full clone) can
//! exhaust memory. [`BoundedMessages`] retains only the most recent
//! [`MAX_IMPORTED_MESSAGES`] messages, dropping older ones, so import memory
//! stays bounded regardless of rollout size.

use crate::provider::Message;
use std::collections::VecDeque;

/// Maximum number of messages retained when importing a Codex session.
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

    /// Log any eviction and consume the buffer into the retained messages.
    pub(crate) fn finish(self, path: &std::path::Path) -> Vec<Message> {
        if self.dropped > 0 {
            tracing::warn!(
                path = %path.display(),
                dropped = self.dropped,
                "Codex import: rollout exceeded message cap, kept most recent messages"
            );
        }
        self.inner.into()
    }
}

#[cfg(test)]
mod tests;
