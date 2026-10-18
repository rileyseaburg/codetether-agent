//! Native AppState field bindings for shared manual watchdog recovery.
use crate::tui::app::state::AppState;
use codetether_watchdog::recovery::{State, Ui};
impl Ui for AppState {
    fn state(&mut self) -> State<'_> {
        State {
            notification: &mut self.watchdog_notification,
            processing: &mut self.processing,
            restarts: &mut self.main_watchdog_restart_count,
            inflight: &mut self.main_inflight_prompt,
            last_event: &mut self.main_last_event_at,
            status: &mut self.status,
            messages: &mut self.messages,
        }
    }
    fn clear_streaming(&mut self) {
        self.clear_streaming_text();
    }
    fn clear_timing(&mut self) {
        self.clear_request_timing();
    }
    fn begin_timing(&mut self) {
        self.begin_request_timing();
    }
    fn scroll(&mut self) {
        self.scroll_to_bottom();
    }
}
