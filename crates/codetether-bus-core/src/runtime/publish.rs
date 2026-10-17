//! publish operations for the shared bus runtime.
use super::*;
impl AgentBus {
    pub fn publish(&self, envelope: BusEnvelope) -> usize {
        registry::apply(&self.registry, &envelope);
        // PRIMARY: persist coordination messages durably (fire-and-forget) so
        // `publish` stays sync; the log is replay's source of truth.
        durable_log::spawn_append_if_durable(self.durable.as_ref(), &envelope);
        // Record into the in-memory ring buffer for on-demand inspection.
        self.recorder.record(&envelope);
        // SECONDARY: live broadcast. Lossy by design; no receivers => dropped.
        self.tx.send(envelope).unwrap_or(0)
    }
}
