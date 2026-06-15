//! Ralph learning publish helper for [`BusHandle`] (split for line budget).

use super::{BusHandle, BusMessage};

impl BusHandle {
    /// Publish learnings from a Ralph iteration so other agents / future
    /// iterations can build on them.
    pub fn publish_ralph_learning(
        &self,
        prd_id: &str,
        story_id: &str,
        iteration: usize,
        learnings: Vec<String>,
        context: serde_json::Value,
    ) -> usize {
        self.send(
            format!("ralph.{prd_id}"),
            BusMessage::RalphLearning {
                prd_id: prd_id.to_string(),
                story_id: story_id.to_string(),
                iteration,
                learnings,
                context,
            },
        )
    }
}
