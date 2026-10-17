//! Application probes feeding shared swarm pre-flight validation.
//!
//! [SwarmValidator] gathers provider/workspace state, then the shared policy
//! assembles and formats the validation report.
mod git_status;
mod provider_check;
mod provider_model;
mod validator;
mod workspace_check;
mod workspace_issues;
mod workspace_status_build;
pub use codetether_swarm_core::validation::{
    IssueCategory, IssueSeverity, ProviderStatus, TokenEstimate, ValidationIssue, ValidationReport,
    WorkspaceStatus,
};
pub use validator::SwarmValidator;
