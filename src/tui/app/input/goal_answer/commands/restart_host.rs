//! Native cancellation and stale prompt cleanup.
use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub runtime: &'a TuiSessionHandle,
}
impl codetether_goal_command::input::Restart for Native<'_> {
    fn cancel(&mut self) {
        self.runtime.request_cancel_current();
    }
    fn clear_watchdog(&mut self) {
        self.app.state.watchdog_notification = None;
    }
    fn clear_inflight(&mut self) {
        self.app.state.main_inflight_prompt = None;
    }
}
