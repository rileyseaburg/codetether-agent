//! Application probes feeding shared swarm pre-flight validation.
//!
//! [SwarmValidator] gathers provider/workspace state, then the shared policy
//! assembles and formats the validation report.
mod host;
pub use codetether_swarm_validation::{
    IssueCategory, IssueSeverity, ProviderStatus, TokenEstimate, ValidationIssue, ValidationReport,
    WorkspaceStatus,
};
/// Swarm preflight validation using the application's provider registry.
pub type SwarmValidator = codetether_swarm_validation::SwarmValidator<host::Native>;
