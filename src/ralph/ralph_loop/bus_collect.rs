//! Drain and format inter-iteration learnings from the agent bus.
use super::*;
impl RalphLoop {
    /// Collect accumulated learnings from the bus for injection into prompts.
    pub(super) fn bus_collect_learnings(&self) -> Vec<String> {
        let Some(ref bus) = self.bus else {
            return Vec::new();
        };
        let prd_id = &self.state.prd.project;
        let mut handle = bus.handle("ralph-collector");
        let envelopes = handle.drain_ralph_learnings(prd_id);
        let mut learnings = Vec::new();
        for env in envelopes {
            match env.message {
                crate::bus::BusMessage::RalphLearning {
                    story_id,
                    iteration,
                    learnings: items,
                    ..
                } => {
                    for l in items {
                        learnings.push(format!("[{story_id} iter {iteration}] {l}"));
                    }
                }
                crate::bus::BusMessage::RalphHandoff {
                    from_story,
                    progress_summary,
                    ..
                } => {
                    learnings.push(format!("[handoff from {from_story}] {progress_summary}"));
                }
                _ => {}
            }
        }
        learnings
    }
}
