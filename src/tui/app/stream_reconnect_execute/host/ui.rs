use super::Native;
use codetether_stream_recovery::retry::State;
impl Native<'_> {
    pub(super) fn state(&mut self) -> State<'_> {
        State {
            recovery: &mut self.app.state.stream_recovery,
            processing: &mut self.app.state.processing,
            inflight: &mut self.app.state.watchdog.inflight_prompt,
            last_event: &mut self.app.state.watchdog.last_event_at,
        }
    }
    pub(super) fn banner(&mut self) {
        crate::tui::app::background::stream_reconnect::push_reconnect_banner(self.app);
    }
    pub(super) fn begin_timing(&mut self) {
        self.app.state.begin_request_timing();
    }
}
