use crate::tui::app::state::App;
use codetether_tui_tick::{Host, Snapshot};
pub(super) struct Native<'a>(pub &'a mut App);
impl Host for Native<'_> {
    fn question_wait(&mut self) {
        crate::tui::app::watchdog::question_wait::tick(self.0);
    }
    fn snapshot(&self) -> Snapshot {
        super::dirty::capture(self.0)
    }
    fn autochat(&mut self) {
        super::super::autochat::drain_autochat(self.0);
    }
    fn voice(&mut self) {
        crate::tui::app::event_handlers::drain_voice_transcription(&mut self.0.state);
    }
    fn ralph(&mut self) -> bool {
        self.0.state.ralph.drain_events()
    }
    fn swarm(&mut self) -> bool {
        crate::tool::swarm_execute::tui_bridge::drain(&mut self.0.state.swarm)
    }
    fn forage(&mut self) -> bool {
        crate::tui::forage_run::drain_forage_updates(self.0)
    }
    fn shell(&mut self) -> bool {
        crate::tui::app::input::shell_bg::drain_shell_events(self.0)
    }
    fn history(&mut self) -> bool {
        crate::tui::app::state::history_page::drain(self.0)
    }
    fn symbols(&mut self) -> bool {
        crate::tui::app::symbols::drain_refresh(self.0)
    }
    fn approvals(&mut self) -> bool {
        super::approval::reconcile(self.0)
    }
    fn mark_redraw(&mut self, changed: bool) {
        self.0.state.needs_redraw |= changed;
    }
}
