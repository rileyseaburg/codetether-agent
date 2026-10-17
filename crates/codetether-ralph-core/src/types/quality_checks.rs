//! QualityChecks serializable contract.
use super::*;

/// Quality checks configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let checks = codetether_ralph_core::QualityChecks::default();
/// assert!(checks.is_empty());
/// ```
pub struct QualityChecks {
    /// Command to run type checking
    #[serde(default)]
    pub typecheck: Option<String>,

    /// Command to run tests
    #[serde(default)]
    pub test: Option<String>,

    /// Command to run linting
    #[serde(default)]
    pub lint: Option<String>,

    /// Command to run build
    #[serde(default)]
    pub build: Option<String>,
}
