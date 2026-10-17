//! Publish one handoff and its matching UI notification.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn announce_relay_turn(
        relay: &ProtocolRelayRuntime,
        from: &str,
        to: &str,
        baton: &str,
        ralph_tx: &Option<mpsc::Sender<RalphEvent>>,
        story_id: &str,
        round: usize,
        max_rounds: usize,
    ) {
        relay.send_handoff(from, to, baton);

        if let Some(tx) = ralph_tx {
            let _ = tx
                .send(RalphEvent::StoryToolCall {
                    story_id: story_id.to_string(),
                    tool_name: format!("relay: @{from} → @{to} (round {round}/{max_rounds})"),
                })
                .await;
        }
    }
}
