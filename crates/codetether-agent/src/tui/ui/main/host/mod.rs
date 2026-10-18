//! Native state bindings for shared TUI render selection.
mod chat;
mod diagnostics;
mod management;
mod overlays;
mod primary;
use crate::tui::app::{session_runtime::SessionView, state::App};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub session: &'a SessionView,
}
impl codetether_tui_render::State for Native<'_> {
    fn view(&self) -> crate::tui::models::ViewMode {
        self.app.state.view.mode
    }
}
