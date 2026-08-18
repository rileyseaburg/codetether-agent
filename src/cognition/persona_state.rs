//! Mutable per-persona runtime state, including budget windows.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{PersonaIdentity, PersonaPolicy, PersonaStatus};

/// Full runtime state for a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaRuntimeState {
    pub identity: PersonaIdentity,
    pub policy: PersonaPolicy,
    pub status: PersonaStatus,
    pub thought_count: u64,
    pub last_tick_at: Option<DateTime<Utc>>,
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
