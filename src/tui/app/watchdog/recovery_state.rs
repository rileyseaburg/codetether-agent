//! Native AppState field bindings for shared manual watchdog recovery.
use crate::tui::app::state::AppState;
use codetether_watchdog::recovery::{State, Ui};
impl Ui for AppState {
    fn state(&mut self) -> State<'_> {
        State {
            notification: &mut self.watchdog.notification,
            processing: &mut self.processing,
            restarts: &mut self.watchdog.restart_count,
            inflight: &mut self.watchdog.inflight_prompt,
            last_event: &mut self.watchdog.last_event_at,
            status: &mut self.status,
            messages: &mut self.messages,
        }
    }
    fn clear_streaming(&mut self) {
        self.message_cache.clear_streaming_text(&mut self.streaming);
    }
    fn clear_timing(&mut self) {
        self.request_timing.clear_request_timing();
    }
    fn begin_timing(&mut self) {
        self.request_timing
            .begin_with_activity(&mut self.watchdog.last_event_at);
    }
    fn scroll(&mut self) {
        self.scroll_to_bottom();
    }
}
