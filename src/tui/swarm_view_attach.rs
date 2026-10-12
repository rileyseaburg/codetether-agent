//! New-run attachment and state reset for the swarm monitor.

use super::SwarmViewState;
use crate::tui::swarm_view::SwarmEvent;
use tokio::sync::mpsc;

impl SwarmViewState {
    /// Attach a live event receiver after discarding prior-run view state.
    pub fn attach_event_rx(&mut self, rx: mpsc::Receiver<SwarmEvent>) {
        *self = Self {
            active: true,
            event_rx: Some(rx),
            ..Self::default()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attaching_a_new_run_discards_completed_run_state() {
        let mut state = SwarmViewState::default();
        state.task = "old task".into();
        state.complete = true;
        state.error = Some("old error".into());
        state.current_stage = 4;
        state.total_stages = 5;
        state.selected_index = 3;
        state.detail_mode = true;
        let (_tx, rx) = mpsc::channel(1);

        state.attach_event_rx(rx);

        assert!(state.active);
        assert!(!state.complete);
        assert!(state.error.is_none());
        assert!(state.task.is_empty());
        assert_eq!(state.current_stage, 0);
        assert_eq!(state.total_stages, 0);
        assert_eq!(state.selected_index, 0);
        assert!(!state.detail_mode);
    }
}
