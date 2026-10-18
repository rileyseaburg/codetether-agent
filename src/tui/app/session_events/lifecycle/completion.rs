//! Native inbox, timing and watchdog completion cleanup.
use super::Native;
impl codetether_session_events::lifecycle::Completion for Native<'_> {
    fn settle_inbox(&mut self) {
        crate::tui::app::inbox::live::settle_from_session(self.app, self.slot);
    }
    fn clear_streaming_text(&mut self) {
        self.app.state.clear_streaming_text();
    }
    fn complete_turn_timing(&mut self) {
        self.app.state.complete_turn_timing();
    }
    fn clear_streaming_start(&mut self) {
        self.app.state.streaming.streaming_start = None;
    }
    fn clear_interlude(&mut self) {
        self.app.state.interlude = None;
    }
    fn clear_watchdog_notification(&mut self) {
        self.app.state.watchdog_notification = None;
    }
    fn clear_inflight_prompt(&mut self) {
        self.app.state.main_inflight_prompt = None;
    }
}
