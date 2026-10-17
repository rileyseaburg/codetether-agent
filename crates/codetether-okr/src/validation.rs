//! Validation failures for OKR entities.

use serde::{Deserialize, Serialize};

/// Validation errors for OKR entities
///
/// # Examples
/// ```
/// use codetether_okr::{OkrValidationError};
/// let value = OkrValidationError::EmptyTitle;
/// assert!(matches!(value, OkrValidationError::EmptyTitle));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrValidationError {
    /// Objective title is empty
    EmptyTitle,

    /// No key results defined
    NoKeyResults,

    /// Key result title is empty
    EmptyKeyResultTitle,

    /// Target value is invalid
    InvalidTargetValue,

    /// Run name is empty
    EmptyRunName,

    /// Invalid status transition
    InvalidStatusTransition,

    /// Run not approved
    NotApproved,
}

impl std::fmt::Display for OkrValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OkrValidationError::EmptyTitle => write!(f, "objective title cannot be empty"),
            OkrValidationError::NoKeyResults => write!(f, "at least one key result is required"),
            OkrValidationError::EmptyKeyResultTitle => {
                write!(f, "key result title cannot be empty")
            }
            OkrValidationError::InvalidTargetValue => {
                write!(f, "target value must be non-negative")
            }
            OkrValidationError::EmptyRunName => write!(f, "run name cannot be empty"),
            OkrValidationError::InvalidStatusTransition => {
                write!(f, "invalid status transition")
            }
            OkrValidationError::NotApproved => write!(f, "run must be approved before starting"),
        }
    }
}
impl std::error::Error for OkrValidationError {}
