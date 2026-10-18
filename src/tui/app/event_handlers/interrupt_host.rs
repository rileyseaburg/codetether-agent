//! Native state and runtime bindings for interruption policy.
use crate::tui::{
    app::{session_runtime::TuiSessionHandle, state::App},
    models::ViewMode,
};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub runtime: &'a TuiSessionHandle,
}
impl codetether_tui_navigation::events::interrupt::Host for Native<'_> {
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    fn in_chat(&self) -> bool {
        self.app.state.view_mode == ViewMode::Chat
    }
    fn request_cancel(&mut self) {
        self.runtime.request_cancel_current();
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
}
