//! Native state and background-check context for incoming approvals.
use crate::tui::app::state::App;
use codetether_approval_queue::ApprovalSnapshot;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_approval_view::request::View for Native<'_> {
    fn reset_preview_scroll(&mut self) {
        self.0.state.approval_preview_scroll = 0;
    }
    fn set_waiting(&mut self, waiting: bool) {
        self.0.state.approval_waiting = waiting;
    }
    fn start_analysis(&mut self, item: &ApprovalSnapshot) {
        codetether_approval_diagnostics::start(
            &mut self.0.state.editor_lsp.manager,
            &self.0.state.cwd_display,
            item,
        );
    }
    fn start_review(&mut self, item: &ApprovalSnapshot) {
        super::review::start(self.0, item);
    }
    fn approval_guidance(&self) -> String {
        crate::tui::ui::trust_status::approval_guidance()
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}
