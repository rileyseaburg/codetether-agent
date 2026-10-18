//! Ralph view lifecycle.
use crate::{RalphEvent, RalphViewState};
use tokio::sync::mpsc;
impl RalphViewState {
    /// Creates an inactive view with no stories or event receiver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Install an event receiver connected to a live `RalphLoop` and reset
    /// the view so it starts clean for the new run.
    pub fn attach_event_rx(&mut self, rx: mpsc::Receiver<RalphEvent>) {
        self.event_rx = Some(rx);
        self.active = true;
        self.complete = false;
        self.final_status = None;
        self.error = None;
    }

    /// Resets the view to an active run with the supplied project and feature.
    pub fn mark_active(&mut self, project: impl Into<String>, feature: impl Into<String>) {
        self.handle_event(RalphEvent::Started {
            project: project.into(),
            feature: feature.into(),
            stories: Vec::new(),
            max_iterations: 0,
        });
    }
}
