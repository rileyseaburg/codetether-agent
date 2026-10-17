//! Forage stop reporting.
use super::*;
impl RunEvents<'_> {
    pub(in crate::forage) async fn stop(&self, cycle: usize) {
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
