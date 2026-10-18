//! Stable per-stream indexing for function calls.
use crate::Encoder;
impl Encoder {
    pub(crate) fn tool_index(&mut self, id: &str) -> usize {
        *self
            .tool_call_indices
            .entry(id.to_owned())
            .or_insert_with(|| {
                let value = self.next_tool_call_index;
                self.next_tool_call_index += 1;
                value
            })
    }
}
