//! Serialized KeyResult data.

use crate::defaults::default_unit;
use crate::defaults::utc_now;
use crate::{KeyResultStatus, KrMetricType, KrOutcome};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A measurable key result within an OKR
///
/// # Examples
/// ```
/// use codetether_okr::{KeyResult};
/// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
/// assert_eq!(value.current_value, 0.0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResult {
    /// Unique identifier for this key result
    pub id: Uuid,

    /// Parent OKR ID
    pub okr_id: Uuid,

    /// Human-readable title
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Target value (numeric for percentage/count metrics)
    pub target_value: f64,

    /// Current value (progress)
    #[serde(default)]
    pub current_value: f64,

    /// Unit of measurement (e.g., "%", "count", "files", "tests")
    #[serde(default = "default_unit")]
    pub unit: String,

    /// Type of metric
    #[serde(default)]
    pub metric_type: KrMetricType,

    /// Current status
    #[serde(default)]
    pub status: KeyResultStatus,

    /// Evidence/outcomes linked to this KR
    #[serde(default)]
    pub outcomes: Vec<KrOutcome>,

    /// Creation timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,
}
