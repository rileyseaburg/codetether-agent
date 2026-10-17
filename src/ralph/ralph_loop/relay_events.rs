//! Forward live tool-call events while a relay session task is running.
use super::*;
impl RalphLoop {
    pub(super) async fn forward_relay_events<T: Send>(
        join: &tokio::task::JoinHandle<T>,
        event_rx: &mut mpsc::Receiver<SessionEvent>,
        ralph_tx: &Option<mpsc::Sender<RalphEvent>>,
        to: &str,
        story_id: &str,
    ) {
        // Forward tool call events to Ralph UI while agent works
        while !join.is_finished() {
            while let Ok(event) = event_rx.try_recv() {
                if let Some(tx) = ralph_tx
                    && let SessionEvent::ToolCallStart { ref name, .. } = event
                {
                    let _ = tx
                        .send(RalphEvent::StoryToolCall {
                            story_id: story_id.to_string(),
                            tool_name: format!("@{to}: {name}"),
                        })
                        .await;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}
