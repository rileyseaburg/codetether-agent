//! Mutable per-persona runtime state, including budget windows.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{PersonaIdentity, PersonaPolicy, PersonaStatus};

/// Full runtime state for a persona.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = PersonaRuntimeState { identity: PersonaIdentity { id: "p1".into(), name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, parent_id: None, depth: 0, created_at: chrono::Utc::now(), tags: vec![] }, policy: PersonaPolicy::default(), status: PersonaStatus::Active, thought_count: 0, last_tick_at: None, updated_at: now, tokens_this_window: 0, compute_ms_this_window: 0, window_started_at: now, last_progress_at: now, budget_paused: false };
/// assert!(!value.budget_paused);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaRuntimeState {
    pub identity: PersonaIdentity,
    /// Resource and lineage limits.
    pub policy: PersonaPolicy,
    /// Current lifecycle state.
    pub status: PersonaStatus,
    /// Number of thoughts produced by the persona.
    pub thought_count: u64,
    /// Most recent processed tick, if any.
    pub last_tick_at: Option<DateTime<Utc>>,
    /// Most recent state update time.
    pub updated_at: DateTime<Utc>,
    /// Tokens consumed in current 60-second window.
    pub tokens_this_window: u32,
    /// Compute milliseconds consumed in current 60-second window.
    pub compute_ms_this_window: u32,
    /// Start of the current budget window.
    pub window_started_at: DateTime<Utc>,
    /// Last time this persona made meaningful progress (not budget-paused or
    /// quorum-waiting).
    pub last_progress_at: DateTime<Utc>,
    /// Whether this persona is currently paused due to budget exhaustion.
    #[serde(default)]
    pub budget_paused: bool,
}
