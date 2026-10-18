//! Owned delta envelope transferred to the blocking writer.
use crate::state::Checkpoint;
/// Serialized dirty suffixes and expected revision for one atomic commit.
/// # Examples
/// ```rust
/// use codetether_session_store::{Batch, Records};
/// let batch = Batch { id: "session-123".into(), revision: 0, header: "{}".into(),
///     messages: Records { start: 0, end: 0, rows: vec![] },
///     tools: Records { start: 0, end: 0, rows: vec![] } };
/// assert_eq!(batch.revision, 0);
/// ```
pub struct Batch {
    /// Session identity.
    pub id: String,
    /// Expected prior revision.
    pub revision: i64,
    /// Serialized replacement header.
    pub header: String,
    /// Dirty message/page suffix.
    pub messages: Records,
    /// Dirty tool-use suffix.
    pub tools: Records,
}
/// One serialized replacement suffix with absolute sequence bounds.
/// # Examples
/// ```rust
/// let rows = codetether_session_store::Records { start: 0, end: 0, rows: vec![] };
/// assert!(rows.rows.is_empty());
/// ```
pub struct Records {
    /// First replaced absolute sequence.
    pub start: usize,
    /// Absolute stream end after replacement.
    pub end: usize,
    /// Serialized replacement records.
    pub rows: Vec<String>,
}
impl Batch {
    /// Test whether this envelope leaves the checkpoint unchanged.
    pub fn unchanged(&self, c: &Checkpoint) -> bool {
        self.revision > 0
            && self.header == c.header
            && self.messages.rows.is_empty()
            && self.tools.rows.is_empty()
            && self.messages.start == self.messages.end
            && self.tools.start == self.tools.end
            && self.messages.end == c.message_end
            && self.tools.end == c.tool_end
    }
}
