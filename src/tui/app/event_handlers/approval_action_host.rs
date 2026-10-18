//! Native fields and lifecycle operations for shared approval actions.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_approval_edit::ApprovalEditSession;
use codetether_approval_view::input::Preview;
use codetether_editor::FileBuffer;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_approval_view::interactive::Host for Native<'_> {
    fn preview(&mut self) -> Preview<'_> {
        Preview {
            chat: self.0.state.view_mode == ViewMode::Chat,
            editing: self.0.state.approval_edit.is_some(),
            help: self.0.state.show_help,
            input: &self.0.state.input,
            scroll: &mut self.0.state.approval_preview_scroll,
        }
    }
    fn run_command(&mut self, command: &str) -> bool {
        crate::tui::app::input::approval_command::run(self.0, command)
    }
    fn editor(&mut self) -> &mut Option<FileBuffer> {
        &mut self.0.state.editor
    }
    fn session(&mut self) -> &mut Option<ApprovalEditSession> {
        &mut self.0.state.approval_edit
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
