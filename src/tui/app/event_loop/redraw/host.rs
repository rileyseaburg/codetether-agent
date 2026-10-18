use super::{App, CrosstermBackend, Instant, SessionView, Terminal};
pub(super) struct Native<'a> {
    pub terminal: &'a mut Terminal<CrosstermBackend<std::io::Stdout>>,
    pub app: &'a mut App,
    pub session: &'a SessionView,
}
impl codetether_tui_tick::redraw::Host for Native<'_> {
    type Error = anyhow::Error;
    fn needs_redraw(&mut self) -> &mut bool {
        &mut self.app.state.needs_redraw
    }
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    fn last_input(&self) -> Option<Instant> {
        self.app.state.last_key_at
    }
    fn draw(&mut self) -> anyhow::Result<bool> {
        crate::tui::app::safe_draw::draw_ui(self.terminal, self.app, self.session)
    }
}
