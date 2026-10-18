//! Serializable model routing metadata; never inferred from generated content.

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Runtime state of the most recently started verifier attempt.
/// Variants cover resolving, running, explicit pass/fail and unavailable execution.
/// # Examples
/// ```
/// use codetether_verifier_state::observation::RunState;
/// assert!(matches!(RunState::Resolving, RunState::Resolving));
/// ```
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Resolving,
    Running,
    Pass,
    Fail,
    Unavailable,
}

/// A routing receipt written by the harness, not the LLM or request caller.
/// # Examples
/// ```
/// let store = std::sync::Arc::new(codetether_verifier_state::observation::ObservationStore::default());
/// let ticket = store.start();
/// let receipt = store.latest().unwrap();
/// assert_eq!(receipt.verification_id, ticket.id());
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub verification_id: String,
    pub identity_source: &'static str,
    pub requested_model: Option<String>,
    pub resolved_provider: Option<String>,
    pub resolved_model: Option<String>,
    pub state: RunState,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}
