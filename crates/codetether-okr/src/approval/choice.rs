//! ApprovalChoice serialized state.

use serde::{Deserialize, Serialize};

/// Approval choice enum
///
/// # Examples
/// ```
/// use codetether_okr::{ApprovalChoice};
/// let value = ApprovalChoice::Approved;
/// assert!(matches!(value, ApprovalChoice::Approved));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalChoice {
    /// Approved to proceed
    Approved,

    /// Denied
    Denied,
}
