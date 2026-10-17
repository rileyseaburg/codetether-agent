//! Pure swarm pre-flight validation and report formatting.
//!
//! [ValidationPolicy] combines supplied [ProviderStatus] and [WorkspaceStatus]
//! observations with configuration, dependency and token checks. [ValidationReport]
//! retains ordered issues and can format a human-readable summary.
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
mod issue;
mod policy;
mod provider_status;
mod report;
mod token_counts;
mod token_estimate;
mod token_issues;
mod token_status;
mod workspace_status;
pub use issue::{IssueCategory, IssueSeverity, ValidationIssue};
pub use policy::ValidationPolicy;
pub use provider_status::ProviderStatus;
pub use report::ValidationReport;
pub use token_status::TokenEstimate;
pub use workspace_status::WorkspaceStatus;
#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod tests;
