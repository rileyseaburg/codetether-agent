//! Provider and Git workspace observations for swarm preflight validation.
//!
//! Supply registry loading through [Host], then construct [SwarmValidator] and
//! call its `validate` method. The validator gathers provider/workspace state
//! before invoking the shared swarm configuration, dependency, and token rules.
mod git_status;
mod host;
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
pub use host::Host;
pub use validator::SwarmValidator;
