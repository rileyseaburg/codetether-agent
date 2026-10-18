use super::RuntimeView;
use crate::MuxRuntimeStatus;
impl RuntimeView<'_> {
    /// Construct an owned report using the existing untitled-session fallback.
    pub fn snapshot(&self) -> MuxRuntimeStatus {
        MuxRuntimeStatus {
            session_id: self.session_id.to_owned(),
            session_title: self
                .title
                .map(str::to_owned)
                .unwrap_or_else(|| "Untitled session".into()),
            processing: self.processing,
            message_count: self.message_count,
            current_tool: self.current_tool.map(str::to_owned),
            needs_interaction: !self.input.trim().is_empty(),
            lagging: self.lagging(),
            principal: self.principal.clone(),
        }
    }
}
