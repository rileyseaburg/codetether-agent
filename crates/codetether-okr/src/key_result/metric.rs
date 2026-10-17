//! KrMetricType serialized state.

use serde::{Deserialize, Serialize};

/// Type of metric for a key result
///
/// # Examples
/// ```
/// use codetether_okr::{KrMetricType};
/// let value = KrMetricType::Progress;
/// assert!(matches!(value, KrMetricType::Progress));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum KrMetricType {
    /// Progress percentage (0-100)
    #[default]
    Progress,

    /// Count of items
    Count,

    /// Boolean completion
    Binary,

    /// Latency/duration (lower is better)
    Latency,

    /// Quality score (0-1 or 0-100)
    Quality,
}
