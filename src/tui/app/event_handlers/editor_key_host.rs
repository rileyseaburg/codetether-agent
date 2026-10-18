//! Native editor field access and external action bindings.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_editor::{EditorInput, FileBuffer};
use std::path::Path;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_editor::interactive::KeyHost for Native<'_> {
    fn in_editor(&self) -> bool {
        self.0.state.view_mode == ViewMode::Editor
    }
    fn approval_editing(&self) -> bool {
        self.0.state.approval_edit.is_some()
    }
    fn handle_approval_action(&mut self, action: &EditorInput) -> bool {
        super::approval_editor::handle(self.0, action)
    }
    fn editor(&mut self) -> &mut Option<FileBuffer> {
        &mut self.0.state.editor
    }
    fn show_chat(&mut self) {
        self.0.state.view_mode = ViewMode::Chat;
    }
    fn reset_scroll(&mut self) {
        self.0.state.editor_scroll = 0;
        self.0.state.editor_hscroll = 0;
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn redraw(&mut self) {
        self.0.state.needs_redraw = true;
    }
    fn open_finder(&mut self, cwd: &Path) {
        crate::tui::app::fuzzy_find::open_fuzzy_find(self.0, cwd);
    }
}
