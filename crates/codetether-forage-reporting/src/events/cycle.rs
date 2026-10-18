//! Forage cycle reporting.
use super::*;
impl RunEvents<'_> {
    /// Record the number of opportunities selected in the cycle audit.
    pub async fn cycle(&self, cycle: usize, top: usize, selected: &[ForageOpportunity]) {
        log_audit(
            AuditCategory::Cognition,
            "forage.cycle",
            AuditOutcome::Success,
            Some(json!({
                "cycle": cycle,
                "selected": selected.len(),
                "top": top,
            })),
            None,
            None,
        )
        .await;
    }
}
