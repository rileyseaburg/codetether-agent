//! Session metadata used during hydration.
use super::Session;
impl codetether_tui_startup::hydrate::Session for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn auto_apply(&self) -> bool {
        self.metadata.auto_apply_edits
    }
    fn allow_network(&self) -> bool {
        self.metadata.allow_network
    }
    fn autocomplete(&self) -> bool {
        self.metadata.slash_autocomplete
    }
    fn use_worktree(&self) -> bool {
        self.metadata.use_worktree
    }
    fn set_allow_network(&mut self, enabled: bool) {
        self.metadata.allow_network = enabled;
    }
}
