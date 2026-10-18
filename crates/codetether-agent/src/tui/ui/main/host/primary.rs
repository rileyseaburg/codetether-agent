//! Native bindings for primary views.
use super::Native;
use ratatui::Frame;
impl codetether_tui_render::Primary for Native<'_> {
    fn sessions(&mut self, frame: &mut Frame) {
        crate::tui::ui::sessions::render_sessions_view(frame, self.app);
    }
    fn swarm(&mut self, frame: &mut Frame) {
        crate::tui::swarm_view::render_swarm_view(frame, &mut self.app.state.swarm, frame.area());
    }
    fn ralph(&mut self, frame: &mut Frame) {
        crate::tui::ralph_view::render_ralph_view(frame, &mut self.app.state.ralph, frame.area());
    }
    fn bus(&mut self, frame: &mut Frame) {
        super::super::bus::render(frame, self.app);
    }
    fn subagents(&mut self, frame: &mut Frame) {
        crate::tui::ui::subagent_view::render(frame, frame.area(), &self.app.state);
    }
    fn model(&mut self, frame: &mut Frame) {
        crate::tui::model_picker::render_model_picker(frame, frame.area(), self.app, self.session);
    }
    fn settings(&mut self, frame: &mut Frame) {
        crate::tui::settings::render_settings(frame, frame.area(), &self.app.state);
    }
}
