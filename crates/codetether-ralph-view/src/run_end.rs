//! Ralph run end event reduction.
use crate::{RalphEvent, RalphViewState};

impl RalphViewState {
    pub(crate) fn apply_run_end(&mut self, event: RalphEvent) {
        match event {
            RalphEvent::StageComplete { .. } => {
                // Could track stage progress if needed
            }
            RalphEvent::Complete {
                status,
                passed: _,
                total: _,
            } => {
                self.complete = true;
                self.final_status = Some(status);
            }
            RalphEvent::Error(err) => {
                self.error = Some(err);
            }
            _ => unreachable!("event routed to incorrect Ralph reducer"),
        }
    }
}
