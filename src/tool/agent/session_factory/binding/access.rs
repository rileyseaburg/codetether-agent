//! Borrow native session settings and tracked message iteration.
use crate::provider::Message;
use crate::session::Session;
use codetether_agent_session::{SessionAccess, Settings};
impl SessionAccess for Session {
    fn settings(&mut self) -> Settings<'_> {
        Settings {
            model: &mut self.metadata.model,
            directory: &mut self.metadata.directory,
            inherited_prior_context_allowed: &mut self.metadata.inherited_prior_context_allowed,
            auto_apply_edits: &mut self.metadata.auto_apply_edits,
            workspace_pinned: self.metadata.workspace_pinned,
        }
    }
    fn messages_mut(&mut self) -> std::slice::IterMut<'_, Message> {
        self.messages.iter_mut()
    }
    fn reset_summary_index(&mut self) {
        self.summary_index = crate::session::index::SummaryIndex::new();
    }
    fn add_message(&mut self, message: Message) {
        Session::add_message(self, message);
    }
}
