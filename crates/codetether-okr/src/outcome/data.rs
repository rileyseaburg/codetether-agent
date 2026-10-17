//! Serialized KrOutcome data.

use crate::KrOutcomeType;
use crate::defaults::utc_now;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Evidence-backed outcome for a key result
///
/// # Examples
/// ```
/// use codetether_okr::{KrOutcome};
/// let value = KrOutcome::new(uuid::Uuid::nil(), "Passing checks");
/// assert_eq!(value.description, "Passing checks");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrOutcome {
    /// Unique identifier for this outcome
    pub id: Uuid,

    /// Key result this outcome belongs to
    pub kr_id: Uuid,

    /// OKR run this outcome is part of
    #[serde(default)]
    pub run_id: Option<Uuid>,

    /// Description of the outcome/evidence
    pub description: String,

    /// Type of outcome
    #[serde(default)]
    pub outcome_type: KrOutcomeType,

    /// Numeric value contributed (if applicable)
    #[serde(default)]
    pub value: Option<f64>,

    /// Evidence links (URLs, file paths, etc.)
    #[serde(default)]
    pub evidence: Vec<String>,

    /// Who/what generated this outcome
    #[serde(default)]
    pub source: String,

    /// Timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,
}
