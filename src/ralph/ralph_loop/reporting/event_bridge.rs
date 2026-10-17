//! Forward swarm events into the Ralph event channel.
use super::*;
impl RalphLoop {
    /// Create a bridge that forwards SwarmEvent → RalphEvent for a given story_id.
    /// Returns the sender to pass to `run_agent_loop` and a join handle for the
    /// forwarding task.
    pub(in crate::ralph::ralph_loop) fn create_swarm_event_bridge(
        ralph_tx: &mpsc::Sender<RalphEvent>,
        story_id: String,
    ) -> (mpsc::Sender<SwarmEvent>, tokio::task::JoinHandle<()>) {
        let (swarm_tx, mut swarm_rx) = mpsc::channel::<SwarmEvent>(100);
        let ralph_tx = ralph_tx.clone();
        let handle = tokio::spawn(async move {
            while let Some(event) = swarm_rx.recv().await {
                let Some(ralph_event) =
                    codetether_ralph_core::events::from_swarm_event(&story_id, event)
                else {
                    continue;
                };
                if ralph_tx.send(ralph_event).await.is_err() {
                    break;
                }
            }
        });
        (swarm_tx, handle)
    }
}
