//! ralph drain operations for the shared bus runtime.
use super::*;
impl BusHandle {
    // ── Ralph helpers ────────────────────────────────────────────────
    // `publish_ralph_*` live in `ralph_publish.rs` (SRP / line budget).

    /// Drain all accumulated Ralph learnings for a PRD (non-blocking).
    pub fn drain_ralph_learnings(&mut self, prd_id: &str) -> Vec<BusEnvelope> {
        let prefix = format!("ralph.{prd_id}");
        let mut out = Vec::new();
        while let Some(env) = self.try_recv() {
            if env.topic.starts_with(&prefix)
                && matches!(
                    &env.message,
                    BusMessage::RalphLearning { .. } | BusMessage::RalphHandoff { .. }
                )
            {
                out.push(env);
            }
        }
        out
    }
}
