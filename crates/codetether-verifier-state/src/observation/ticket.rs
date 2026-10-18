//! Per-run routing identity survives settings changes and records cancellation.

use super::{Observation, ObservationStore, RunState};
use crate::Verdict;
use std::sync::Arc;

/// One verification attempt; dropping unfinished work records unavailability.
/// # Examples
/// ```
/// let store = std::sync::Arc::new(codetether_verifier_state::observation::ObservationStore::default());
/// let mut ticket = store.start();
/// ticket.resolved("provider", "model");
/// assert_eq!(ticket.identity(), "provider/model");
/// ```
pub struct Ticket {
    pub(super) store: Arc<ObservationStore>,
    pub(super) record: Observation,
}

impl Ticket {
    /// Return the stable verification ID.
    pub fn id(&self) -> &str {
        &self.record.verification_id
    }
    /// Record the requested model before provider resolution.
    pub fn requested(&mut self, requested: String) {
        self.record.requested_model = Some(requested);
        self.store.publish(&self.record);
    }
    /// Record actual provider resolution and mark the attempt running.
    pub fn resolved(&mut self, provider: &str, model: &str) {
        self.record.resolved_provider = Some(provider.into());
        self.record.resolved_model = Some(model.into());
        self.record.state = RunState::Running;
        self.store.publish(&self.record);
    }
    /// Return captured provider/model, or the unresolved marker.
    pub fn identity(&self) -> String {
        match (&self.record.resolved_provider, &self.record.resolved_model) {
            (Some(provider), Some(model)) => format!("{provider}/{model}"),
            _ => "unresolved-verifier".into(),
        }
    }
    /// Record a verdict or execution error and completion timestamp.
    pub fn finish(&mut self, report: &anyhow::Result<String>) {
        self.record.state = match report.as_deref().map(Verdict::parse) {
            Ok(Verdict::Pass) => RunState::Pass,
            Ok(Verdict::Fail { .. }) => RunState::Fail,
            Ok(Verdict::Unavailable { .. }) | Err(_) => RunState::Unavailable,
        };
        self.record.finished_at = Some(chrono::Utc::now());
        self.store.publish(&self.record);
    }
}

impl Drop for Ticket {
    fn drop(&mut self) {
        if self.record.finished_at.is_none() {
            self.finish(&Err(anyhow::anyhow!("verification cancelled")));
        }
    }
}
