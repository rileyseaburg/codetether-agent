//! Serializable model routing metadata; never inferred from generated content.

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Runtime state of the most recently started verifier attempt.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RunState {
    Resolving,
    Running,
    Pass,
    Fail,
    Unavailable,
}

/// A routing receipt written by the harness, not the LLM or request caller.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Observation {
    pub(crate) verification_id: String,
    pub(crate) identity_source: &'static str,
    pub(crate) requested_model: Option<String>,
    pub(crate) resolved_provider: Option<String>,
    pub(crate) resolved_model: Option<String>,
    pub(crate) state: RunState,
    pub(crate) started_at: DateTime<Utc>,
    pub(crate) finished_at: Option<DateTime<Utc>>,
}
