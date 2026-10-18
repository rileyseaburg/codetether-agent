use crate::tui::app::state::App;
use codetether_watchdog::recovery::{State, Ui};
impl Ui for App {
    fn state(&mut self) -> State<'_> {
        State {
            notification: &mut self.state.watchdog_notification,
            processing: &mut self.state.processing,
            restarts: &mut self.state.main_watchdog_restart_count,
            inflight: &mut self.state.main_inflight_prompt,
            last_event: &mut self.state.main_last_event_at,
            status: &mut self.state.status,
            messages: &mut self.state.messages,
        }
    }
    fn clear_streaming(&mut self) {
        self.state.clear_streaming_text();
    }
    fn clear_timing(&mut self) {
        self.state.clear_request_timing();
    }
    fn begin_timing(&mut self) {
        self.state.begin_request_timing();
    }
    fn scroll(&mut self) {
        self.state.scroll_to_bottom();
    }
}
