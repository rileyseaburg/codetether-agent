//! Forward swarm events into the story-scoped Ralph UI channel.
use codetether_ralph_core::events::RalphEvent;
use codetether_swarm_core::SwarmEvent;
use tokio::sync::mpsc;
/// Create the original bounded event channel and forwarding task.
/// # Arguments
/// Receives the Ralph event sender and story identity.
/// # Returns
/// A swarm-event sender and the forwarding task handle.
/// # Examples
/// Caller retains the handle for the desired story lifetime.
pub fn event_bridge(
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
