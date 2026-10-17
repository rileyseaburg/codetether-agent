//! OkrStatus serialized state.

use serde::{Deserialize, Serialize};

/// OKR status enum
///
/// # Examples
/// ```
/// use codetether_okr::{OkrStatus};
/// let value = OkrStatus::Draft;
/// assert!(matches!(value, OkrStatus::Draft));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OkrStatus {
    /// Draft - not yet approved
    #[default]
    Draft,

    /// Active - approved and in progress
    Active,

    /// Completed - all key results achieved
    Completed,

    /// Cancelled - abandoned before completion
    Cancelled,

    /// OnHold - temporarily paused
    OnHold,
}
