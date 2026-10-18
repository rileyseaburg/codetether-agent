use super::RuntimeView;
use crate::MuxRuntimeStatus;
impl RuntimeView<'_> {
    /// Whether the prior owned status contains the same projected facts.
    pub fn unchanged(&self, last: &MuxRuntimeStatus) -> bool {
        last.session_id == self.session_id
            && last.session_title == self.title.unwrap_or("Untitled session")
            && last.processing == self.processing
            && last.message_count == self.message_count
            && last.current_tool.as_deref() == self.current_tool
            && last.needs_interaction == !self.input.trim().is_empty()
            && last.lagging == self.lagging()
            && last.principal == *self.principal
    }
}
