//! Swarm run progress event reduction.
use crate::{SwarmEvent, SwarmViewState};
impl SwarmViewState {
    pub(crate) fn apply_run_progress(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::StageComplete { stage, .. } => {
                self.current_stage = stage + 1;
            }
            SwarmEvent::Complete { success: _, stats } => {
                (self.active, self.stats, self.complete) = (false, Some(stats), true);
            }
            SwarmEvent::Error(err) => {
                (self.active, self.complete, self.error) = (false, true, Some(err));
            }
            _ => unreachable!("event routed to incorrect swarm reducer"),
        }
    }
}
