//! Swarm run start event reduction.
use crate::{SwarmEvent, SwarmViewState};
use ratatui::widgets::ListState;
impl SwarmViewState {
    pub(crate) fn apply_run_start(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::Started {
                task,
                total_subtasks,
            } => {
                self.active = true;
                self.task = task;
                self.subtasks.clear();
                self.current_stage = 0;
                self.complete = false;
                self.error = None;
                self.selected_index = 0;
                self.detail_mode = false;
                self.detail_scroll = 0;
                self.list_state = ListState::default();
                // Pre-allocate
                self.subtasks.reserve(total_subtasks);
            }
            SwarmEvent::Decomposed { subtasks } => {
                self.subtasks = subtasks;
                self.total_stages = self.subtasks.iter().map(|s| s.stage).max().unwrap_or(0) + 1;
                if !self.subtasks.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            _ => unreachable!("event routed to incorrect swarm reducer"),
        }
    }
}
