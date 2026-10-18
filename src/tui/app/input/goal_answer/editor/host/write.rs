//! Apply ordered native editor and view updates.
use super::Editor;
use crate::tui::models::ViewMode;
impl codetether_goal_editor::Write for Editor<'_> {
    fn buffer_mut(&mut self) -> Option<&mut codetether_editor::FileBuffer> {
        self.app.state.editor.as_mut()
    }
    fn set_buffer(&mut self, buffer: Option<codetether_editor::FileBuffer>) {
        self.app.state.editor = buffer;
    }
    fn set_draft(&mut self, draft: Option<codetether_goal_editor::Draft>) {
        self.app.state.goal_editor = draft;
    }
    fn reset_scroll(&mut self) {
        self.app.state.editor_scroll = 0;
        self.app.state.editor_hscroll = 0;
    }
    fn clear_hover(&mut self) {
        self.app.state.editor_lsp.clear_hover();
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
    fn show_editor(&mut self) {
        self.app.state.set_view_mode(ViewMode::Editor);
    }
    fn show_chat(&mut self) {
        self.app.state.set_view_mode(ViewMode::Chat);
    }
    fn status(&mut self, text: String) {
        self.app.state.status = text;
    }
    fn redraw(&mut self) {
        self.app.state.needs_redraw = true;
    }
}
