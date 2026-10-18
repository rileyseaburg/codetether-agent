//! Native metadata projection for interactive session selection.
use crate::session::Session;
impl codetether_session_resume::interactive::Session for Session {
    fn attach_global_bus_if_missing(&mut self) {
        Session::attach_global_bus_if_missing(self);
    }
    fn auto_apply_edits(&self) -> bool {
        self.metadata.auto_apply_edits
    }
    fn use_worktree(&self) -> bool {
        self.metadata.use_worktree
    }
    fn display_label(&self) -> String {
        Session::display_label(self)
    }
    fn visible_messages(&self) -> usize {
        self.messages.len()
    }
}
