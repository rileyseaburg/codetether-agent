//! Existing frontend fields and file-opening operations.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_chat_layout::ChatHit;
use codetether_chat_model::ChatMessage;
use std::path::Path;
pub(in crate::tui::app::event_handlers) struct Native<'a>(pub &'a mut App);
impl codetether_chat_files::Host for Native<'_> {
    fn in_chat(&self) -> bool {
        self.0.state.view_mode == ViewMode::Chat
    }
    fn messages(&self) -> &[ChatMessage] {
        &self.0.state.messages
    }
    fn chat_hit(&self) -> &ChatHit {
        &self.0.state.chat_hit
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn set_diff(&mut self, diff: String) {
        self.0.state.git.diff_stat = diff;
    }
    fn set_captured_at(&mut self, time: String) {
        self.0.state.git.captured_at = time;
    }
    fn save_scroll_state(&mut self) {
        crate::tui::app::state::view::transition(&mut self.0.state).save_scroll_state();
    }
    fn show_git(&mut self) {
        self.0.state.view_mode = ViewMode::Git;
    }
    fn open_viewer(&mut self, cwd: &Path, file: &Path) {
        crate::tui::app::file_picker::open_file_in_viewer(self.0, cwd, file);
    }
    fn open_editor(&mut self, cwd: &Path, file: &Path) {
        crate::tui::app::fuzzy_find::open_path_in_editor(self.0, cwd, file);
    }
}
