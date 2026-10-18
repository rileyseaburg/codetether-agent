//! Concrete native session slot with a TUI metadata snapshot.
use super::SessionView;
use crate::provenance::RuntimePrincipal;
use crate::session::Session;
pub(crate) type SessionSlot = codetether_session_runtime::slot::SessionSlot<Session>;
impl codetether_session_runtime::slot::ViewSource for Session {
    type View = SessionView;
    fn snapshot(&self) -> SessionView {
        SessionView {
            id: self.id.clone(),
            title: self.title.clone(),
            model: self.metadata.model.clone(),
            directory: self.metadata.directory.clone(),
            auto_apply_edits: self.metadata.auto_apply_edits,
            allow_network: self.metadata.allow_network,
            slash_autocomplete: self.metadata.slash_autocomplete,
            use_worktree: self.metadata.use_worktree,
            message_count: self.messages.len(),
            principal: RuntimePrincipal::for_session(
                &self.agent,
                self.metadata.provenance.as_ref(),
            ),
        }
    }
}
