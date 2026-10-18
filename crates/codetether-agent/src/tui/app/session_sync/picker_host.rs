//! Native listing and picker field bindings for refresh.
use crate::session::SessionSummary;
use crate::tui::app::state::App;
use codetether_session_picker::{
    SessionPickerState,
    refresh::{Source, View},
};
use std::path::Path;
pub(super) struct Catalog;
impl Source for Catalog {
    async fn list(cwd: &Path) -> anyhow::Result<Vec<SessionSummary>> {
        crate::session::list_all_sessions_for_directory(cwd).await
    }
}
pub(super) struct Picker<'a>(pub(super) &'a mut App);
impl View for Picker<'_> {
    fn picker(&mut self) -> &mut SessionPickerState {
        &mut self.0.state.session_picker
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}
