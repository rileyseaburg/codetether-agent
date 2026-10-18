//! Compatibility wrapper for shared story-scoped event forwarding.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn create_swarm_event_bridge(
        ralph_tx: &mpsc::Sender<RalphEvent>,
        story_id: String,
    ) -> (mpsc::Sender<SwarmEvent>, tokio::task::JoinHandle<()>) {
        codetether_ralph_agent::event_bridge(ralph_tx, story_id)
    }
}
