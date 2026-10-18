//! Bind interactive Ralph presentation to the native TUI.
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use codetether_ralph_core::events::{RalphEvent, RalphStoryInfo};
use tokio::sync::mpsc;
pub(super) struct CommandView<'a>(pub(super) &'a mut App);
impl codetether_ralph_cli::interactive::View for CommandView<'_> {
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn push_system_message(&mut self, message: String) {
        super::push_system_message(self.0, message);
    }
    fn attach_event_rx(&mut self, events: mpsc::Receiver<RalphEvent>) {
        self.0.state.ralph.attach_event_rx(events);
    }
    fn show_monitor(&mut self) {
        self.0.state.set_view_mode(ViewMode::Ralph);
    }
    fn stories(&self) -> &[RalphStoryInfo] {
        &self.0.state.ralph.stories
    }
    fn current_iteration(&self) -> usize {
        self.0.state.ralph.current_iteration
    }
    fn max_iterations(&self) -> usize {
        self.0.state.ralph.max_iterations
    }
}
