//! Native bindings for management views.
use super::Native;
use ratatui::Frame;
impl codetether_tui_render::Management for Native<'_> {
    fn file_picker(&mut self, frame: &mut Frame) {
        crate::tui::app::file_picker::render_file_picker(frame, frame.area(), self.app);
    }
    fn audit(&mut self, frame: &mut Frame) {
        crate::tui::audit_view::render_audit_view(frame, &mut self.app.state.audit, frame.area());
    }
    fn git(&mut self, frame: &mut Frame) {
        crate::tui::git_view::render_git_view(
            frame,
            frame.area(),
            &self.app.state.git,
            &self.app.state.status,
        );
    }
    fn audit_loop(&mut self, frame: &mut Frame) {
        crate::tui::audit_loop_view::render_audit_loop_view(
            frame,
            &self.app.state.audit_loop,
            frame.area(),
            &self.app.state.status,
        );
    }
    fn editor(&mut self, frame: &mut Frame) {
        crate::tui::ui::goal_editor::render(frame, self.app);
    }
}
