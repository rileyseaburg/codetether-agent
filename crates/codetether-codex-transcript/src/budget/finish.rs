//! Finish a bounded transcript and report any discarded prefix.
use super::{BoundedMessages, Message};
impl BoundedMessages {
    /// Log any eviction and consume the buffer into the retained messages.
    pub(crate) fn finish(self, path: &std::path::Path) -> Vec<Message> {
        if self.dropped > 0 {
            tracing::warn!(target: "codetether_agent::session::codex_import::budget",
                path = %path.display(),
                dropped = self.dropped,
                "Codex import: rollout exceeded message cap, kept most recent messages"
            );
        }
        self.inner.into()
    }
}
