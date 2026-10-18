//! Forage stop reporting.
use super::*;
impl RunEvents<'_> {
    /// Announce shutdown and audit the completed cycle count.
    pub async fn stop(&self, cycle: usize) {
        let _ = self.bus_handle.announce_shutdown();
        log_audit(
            AuditCategory::Cognition,
            "forage.stop",
            AuditOutcome::Success,
            Some(json!({ "cycles": cycle })),
            None,
            None,
        )
        .await;
    }
}
