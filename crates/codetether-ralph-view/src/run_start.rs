//! Ralph run start event reduction.
use crate::{RalphEvent, RalphViewState};
use ratatui::widgets::ListState;
impl RalphViewState {
    pub(crate) fn apply_run_start(&mut self, event: RalphEvent) {
        match event {
            RalphEvent::Started {
                project,
                feature,
                stories,
                max_iterations,
            } => {
                self.active = true;
                self.project = project;
                self.feature = feature;
                self.stories = stories;
                self.max_iterations = max_iterations;
                self.current_iteration = 0;
                self.complete = false;
                self.final_status = None;
                self.error = None;
                self.selected_index = 0;
                self.detail_mode = false;
                self.detail_scroll = 0;
                self.list_state = ListState::default();
                if !self.stories.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            RalphEvent::IterationStarted {
                iteration,
                max_iterations,
            } => {
                self.current_iteration = iteration;
                self.max_iterations = max_iterations;
            }
            _ => unreachable!("event routed to incorrect Ralph reducer"),
        }
    }
}
