use super::{ProviderStatus, TokenEstimate, ValidationIssue, WorkspaceStatus};

/// Result of pre-flight validation.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// # use codetether_swarm_core::validation::{ProviderStatus, WorkspaceStatus, ValidationPolicy};
/// # fn example(provider: ProviderStatus, workspace: WorkspaceStatus) {
/// let report = ValidationPolicy::new(Default::default()).validate(&[], provider, workspace, vec![]);
/// assert!(!report.format().is_empty());
/// # }
/// ```
pub struct ValidationReport {
    /// Whether all checks passed.
    pub is_valid: bool,
    /// Validation issues found during pre-flight checks.
    pub issues: Vec<ValidationIssue>,
    /// Estimated token usage for swarm execution.
    pub estimated_tokens: TokenEstimate,
    /// Provider availability status.
    pub provider_status: ProviderStatus,
    /// Workspace/git state summary.
    pub workspace_status: WorkspaceStatus,
}
