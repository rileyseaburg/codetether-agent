//! Read the current native editor state.
use super::Editor;
impl codetether_goal_editor::Read for Editor<'_> {
    type Journal = super::journal::Native;
    fn draft(&self) -> Option<&codetether_goal_editor::Draft> {
        self.app.state.goal_editor.as_ref()
    }
    fn buffer(&self) -> Option<&codetether_editor::FileBuffer> {
        self.app.state.editor.as_ref()
    }
    fn approval_open(&self) -> bool {
        self.app.state.approval_edit.is_some()
    }
}
