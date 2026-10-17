//! Serialized Okr data.

use crate::defaults::utc_now;
use crate::{KeyResult, OkrStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A high-level objective with associated key results
///
/// # Examples
/// ```
/// use codetether_okr::{Okr};
/// let mut value = Okr::new("Reliability", "Reduce failures");
/// assert_eq!(value.title, "Reliability");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Okr {
    /// Unique identifier for this OKR
    pub id: Uuid,

    /// Human-readable title of the objective
    pub title: String,

    /// Detailed description of what this objective aims to achieve
    pub description: String,

    /// Current status of the OKR
    #[serde(default)]
    pub status: OkrStatus,

    /// Key results that measure success
    #[serde(default)]
    pub key_results: Vec<KeyResult>,

    /// Owner of this OKR (user ID or team)
    #[serde(default)]
    pub owner: Option<String>,

    /// Tenant ID for multi-tenant isolation
    #[serde(default)]
    pub tenant_id: Option<String>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Creation timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,

    /// Target completion date
    #[serde(default)]
    pub target_date: Option<DateTime<Utc>>,
}
