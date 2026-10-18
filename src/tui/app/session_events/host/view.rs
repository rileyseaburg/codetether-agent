//! Native activity clock and final presentation state.
use super::{Native, SessionEvent};
impl codetether_session_events::pipeline::View for Native<'_> {
    fn last_event_at(&mut self) -> &mut Option<std::time::Instant> {
        &mut self.app.state.main_last_event_at
    }
    fn auto_follow(&self) -> bool {
        self.app.state.chat_viewport.chat_auto_follow
    }
    fn scroll_to_bottom(&mut self) {
        self.app.state.scroll_to_bottom();
    }
    fn trim(&mut self) {
        super::super::retention::trim(self.app);
    }
    fn context(&mut self, event: SessionEvent) {
        super::super::context::handle_event(self.app, event);
    }
}
