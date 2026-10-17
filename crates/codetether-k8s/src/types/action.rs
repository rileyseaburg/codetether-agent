//! DeployAction data contract.

use serde::{Deserialize, Serialize};

/// Result of a self-deployment action.
///
/// # Examples
/// ```
/// use codetether_k8s::{DeployAction};
/// let action = DeployAction { action: "scale:2".into(), success: true,
///     message: "Scaled".into(), timestamp: "2026-01-01T00:00:00Z".into() };
/// assert!(action.success);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAction {
    pub action: String,
    pub success: bool,
    pub message: String,
    pub timestamp: String,
}
