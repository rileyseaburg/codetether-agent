//! KeyResultStatus serialized state.

use serde::{Deserialize, Serialize};

/// Status of a key result
///
/// # Examples
/// ```
/// use codetether_okr::{KeyResultStatus};
/// let value = KeyResultStatus::Pending;
/// assert!(matches!(value, KeyResultStatus::Pending));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum KeyResultStatus {
    /// Not yet started
    #[default]
    Pending,

    /// Actively being worked on
    InProgress,

    /// Achieved the target
    Completed,

    /// At risk of missing target
    AtRisk,

    /// Failed to achieve target
    Failed,
}
