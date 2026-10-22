//! Bounded, durable observations used across prompt and goal continuations.
use serde::{Deserialize, Serialize};

/// Persisted call/result evidence, independent of per-turn execution counters.
/// New observations are evidence of investigation, not proof of task completion.
/// # Examples
/// ```
/// let ledger = codetether_repeat_guard::ProgressLedger::default();
/// assert_eq!(ledger.observations, 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProgressLedger {
    /// Number of observed tool results, including repeated and failed results.
    pub observations: u64,
    /// Number of newly observed call/result combinations across the session.
    pub novel_observations: u64,
    /// Repeated completed results since the latest new observation or guidance.
    pub stagnant_results: u32,
    #[serde(default)]
    pub(super) epoch: u64,
    pub(super) review: super::review::Review,
    #[serde(default)]
    pub(super) pending: Vec<Pending>,
    #[serde(default)]
    pub(super) records: Vec<Record>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Pending {
    pub id: String,
    pub signature: String,
    pub tool: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Record {
    pub signature: String,
    pub response_hash: String,
    pub failed: bool,
    pub repeats: u32,
    pub epoch: u64,
    pub source_turn: usize,
}
