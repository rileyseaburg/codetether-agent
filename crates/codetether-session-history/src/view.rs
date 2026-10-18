//! Immutable transcript access and size queries.
use crate::History;
use codetether_provider_core::Message;
impl History<'_> {
    /// Borrow the history as an immutable slice for reads.
    pub fn view(&self) -> &[Message] {
        self.buf
    }

    /// The number of entries currently in the history.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// `true` when the history is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}
