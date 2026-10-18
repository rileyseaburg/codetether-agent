//! Native state bindings for the shared approval editor workflow.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_approval_edit::ApprovalEditSession;
use codetether_editor::FileBuffer;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_approval_edit::interactive::Host for Native<'_> {
    fn editor(&mut self) -> &mut Option<FileBuffer> {
        &mut self.0.state.editor
    }
    fn session(&mut self) -> &mut Option<ApprovalEditSession> {
        &mut self.0.state.approval_edit
    }
    fn reset_scroll(&mut self) {
        self.0.state.editor_scroll = 0;
        self.0.state.editor_hscroll = 0;
    }
    fn show_chat(&mut self) {
        self.0.state.set_view_mode(ViewMode::Chat);
    }
    fn reset_preview_scroll(&mut self) {
        self.0.state.approval_preview_scroll = 0;
    }
    fn set_waiting(&mut self, waiting: bool) {
        self.0.state.approval_waiting = waiting;
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn redraw(&mut self) {
        self.0.state.needs_redraw = true;
    }
}
