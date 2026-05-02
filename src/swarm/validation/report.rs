use super::{ProviderStatus, TokenEstimate, ValidationIssue, WorkspaceStatus};

/// Result of pre-flight validation.
#[derive(Debug, Clone)]
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
