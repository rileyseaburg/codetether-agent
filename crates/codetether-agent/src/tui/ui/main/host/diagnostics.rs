//! Native bindings for diagnostics views.
use super::Native;
use ratatui::Frame;
impl codetether_tui_render::Diagnostics for Native<'_> {
    fn lsp(&mut self, frame: &mut Frame) {
        crate::tui::lsp::render_lsp(
            frame,
            frame.area(),
            &self.app.state.cwd_display,
            &self.app.state.status,
        );
    }
    fn rlm(&mut self, frame: &mut Frame) {
        crate::tui::rlm::render_rlm(
            frame,
            frame.area(),
            &self.app.state.cwd_display,
            &self.app.state.status,
            self.app.state.session_picker.sessions.len(),
            self.app.state.session_picker.selected_session,
        );
    }
    fn latency(&mut self, frame: &mut Frame) {
        crate::tui::latency::render_latency(frame, frame.area(), self.app);
    }
    fn transport(&mut self, frame: &mut Frame) {
        crate::tui::transport_view::render_transport(frame, frame.area());
    }
    fn protocol(&mut self, frame: &mut Frame) {
        crate::tui::protocol_registry_view::render_protocol_registry(frame, self.app, frame.area());
    }
    fn inspector(&mut self, frame: &mut Frame) {
        crate::tui::ui::inspector::render_inspector_view(frame, self.app);
    }
}
