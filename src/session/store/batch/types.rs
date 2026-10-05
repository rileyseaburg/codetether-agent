//! Owned delta envelope transferred to the blocking writer.
use super::super::state::Checkpoint;
pub(in crate::session::store) struct Batch {
    pub id: String,
    pub revision: i64,
    pub header: String,
    pub messages: Records,
    pub tools: Records,
}
pub(in crate::session::store) struct Records {
    pub start: usize,
    pub end: usize,
    pub rows: Vec<String>,
}
impl Batch {
    pub(in crate::session::store) fn unchanged(&self, c: &Checkpoint) -> bool {
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
