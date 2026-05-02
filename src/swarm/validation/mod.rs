//! Swarm pre-flight validation.
//!
//! This module validates swarm execution configuration before startup so
//! common issues produce actionable diagnostics.

mod config_issues;
mod configuration;
mod dependencies;
mod dependency_cycle;
mod dependency_graph;
mod dependency_issues;
mod format_issue;
mod format_report;
mod format_tokens;
mod format_workspace;
mod git_status;
mod issue;
mod provider_check;
mod provider_model;
mod provider_status;
mod report;
mod token_counts;
mod token_estimate;
mod token_issues;
mod token_status;
mod validator;
mod workspace_check;
mod workspace_issues;
mod workspace_status;
mod workspace_status_build;

pub use issue::{IssueCategory, IssueSeverity, ValidationIssue};
pub use provider_status::ProviderStatus;
pub use report::ValidationReport;
pub use token_status::TokenEstimate;
pub use validator::SwarmValidator;
pub use workspace_status::WorkspaceStatus;

#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod tests;
