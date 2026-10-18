//! Native editor and finder state bindings for shared opening behavior.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_editor::FileBuffer;
use codetether_fuzzy_finder::FuzzyFindState;
pub(super) struct EditorView<'a>(pub(super) &'a mut App);
impl codetether_editor::interactive::View for EditorView<'_> {
    fn finder(&mut self) -> &mut Option<FuzzyFindState> {
        &mut self.0.state.fuzzy_find
    }
    fn set_buffer(&mut self, buffer: FileBuffer) {
        self.0.state.editor = Some(buffer);
    }
    fn reset_editor_scroll(&mut self) {
        self.0.state.editor_scroll = 0;
        self.0.state.editor_hscroll = 0;
    }
    fn show_editor(&mut self) {
        self.0.state.set_view_mode(ViewMode::Editor);
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}
