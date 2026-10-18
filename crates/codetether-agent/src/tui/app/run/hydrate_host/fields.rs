//! Native startup scalar and workspace fields.
use super::Native;
use crate::tui::models::WorkspaceSnapshot;
impl codetether_tui_startup::hydrate::Fields for Native<'_> {
    fn set_workspace(&mut self, workspace: WorkspaceSnapshot) {
        self.0.state.workspace = workspace;
    }
    fn set_auto_apply(&mut self, enabled: bool) {
        self.0.state.preferences.auto_apply_edits = enabled;
    }
    fn set_allow_network(&mut self, enabled: bool) {
        self.0.state.preferences.allow_network = enabled;
    }
    fn set_autocomplete(&mut self, enabled: bool) {
        self.0.state.preferences.slash_autocomplete = enabled;
    }
    fn set_use_worktree(&mut self, enabled: bool) {
        self.0.state.preferences.use_worktree = enabled;
    }
    fn set_session_id(&mut self, id: String) {
        self.0.state.session_id = Some(id);
    }
    fn set_cwd(&mut self, cwd: String) {
        self.0.state.cwd_display = cwd;
    }
    fn set_peer_ready(&mut self, ready: bool) {
        self.0.state.peer_endpoint_ready = ready;
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}
