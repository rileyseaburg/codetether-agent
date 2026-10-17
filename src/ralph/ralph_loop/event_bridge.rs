//! Forward swarm events into the Ralph event channel.
use super::*;
impl RalphLoop {
    /// Create a bridge that forwards SwarmEvent → RalphEvent for a given story_id.
    /// Returns the sender to pass to `run_agent_loop` and a join handle for the
    /// forwarding task.
    pub(super) fn create_swarm_event_bridge(
        ralph_tx: &mpsc::Sender<RalphEvent>,
        story_id: String,
    ) -> (mpsc::Sender<SwarmEvent>, tokio::task::JoinHandle<()>) {
        let (swarm_tx, mut swarm_rx) = mpsc::channel::<SwarmEvent>(100);
        let ralph_tx = ralph_tx.clone();
        let handle = tokio::spawn(async move {
            while let Some(event) = swarm_rx.recv().await {
                let ralph_event = match event {
                    SwarmEvent::AgentToolCall { tool_name, .. } => RalphEvent::StoryToolCall {
                        story_id: story_id.clone(),
                        tool_name,
                    },
                    SwarmEvent::AgentToolCallDetail { detail, .. } => {
                        RalphEvent::StoryToolCallDetail {
                            story_id: story_id.clone(),
                            detail,
                        }
                    }
                    SwarmEvent::AgentMessage { entry, .. } => RalphEvent::StoryMessage {
                        story_id: story_id.clone(),
                        entry,
                    },
                    SwarmEvent::AgentOutput { output, .. } => RalphEvent::StoryOutput {
                        story_id: story_id.clone(),
                        output,
                    },
                    SwarmEvent::AgentError { error, .. } => RalphEvent::StoryError {
                        story_id: story_id.clone(),
                        error,
                    },
                    _ => continue, // Skip swarm-specific events
                };
                if ralph_tx.send(ralph_event).await.is_err() {
                    break;
                }
            }
        });
        (swarm_tx, handle)
    }
}
