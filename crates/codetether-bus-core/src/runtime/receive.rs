//! receive operations for the shared bus runtime.
use super::*;
impl BusHandle {
    // ── Voice helpers live in `voice_handle.rs` (SRP / line budget). ──
    // ── Agent-speech helpers live in `speech.rs`. ────────────────────

    /// Receive the next envelope (blocks until available).
    pub async fn recv(&mut self) -> Option<BusEnvelope> {
        loop {
            match self.rx.recv().await {
                Ok(env) => return Some(env),
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!(
                        agent_id = %self.agent_id,
                        skipped = n,
                        "Bus handle lagged, skipping messages"
                    );
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }
}
