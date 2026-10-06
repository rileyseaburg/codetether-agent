//! Per-run routing identity survives settings changes and records cancellation.

use super::{Observation, ObservationStore, RunState};
use crate::tool::goal::verify::Verdict;
use std::sync::Arc;

pub(crate) struct Ticket {
    pub(super) store: Arc<ObservationStore>,
    pub(super) record: Observation,
}

impl Ticket {
    pub(crate) fn id(&self) -> &str {
        &self.record.verification_id
    }
    pub(crate) fn requested(&mut self, requested: String) {
        self.record.requested_model = Some(requested);
        self.store.publish(&self.record);
    }
    pub(crate) fn resolved(&mut self, provider: &str, model: &str) {
        self.record.resolved_provider = Some(provider.into());
        self.record.resolved_model = Some(model.into());
        self.record.state = RunState::Running;
        self.store.publish(&self.record);
    }
    pub(crate) fn identity(&self) -> String {
        match (&self.record.resolved_provider, &self.record.resolved_model) {
            (Some(provider), Some(model)) => format!("{provider}/{model}"),
            _ => "unresolved-verifier".into(),
        }
    }
    pub(crate) fn finish(&mut self, report: &anyhow::Result<String>) {
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
