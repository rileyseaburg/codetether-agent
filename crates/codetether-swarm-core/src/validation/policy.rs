use super::*;
use crate::{SubTask, SwarmConfig};
/// Pure pre-flight checks evaluated after host provider and workspace probes.
///
/// # Examples
/// ```
/// let policy = codetether_swarm_core::validation::ValidationPolicy::new(Default::default());
/// ```
pub struct ValidationPolicy {
    pub(super) config: SwarmConfig,
}
impl ValidationPolicy {
    /// Construct checks using the supplied execution configuration.
    pub fn new(config: SwarmConfig) -> Self {
        Self { config }
    }
    /// Check configuration, dependencies and token estimates, retaining probe issues first.
    ///
    /// # Arguments
    /// * `subtasks` - Planned tasks to inspect.
    /// * `provider_status` - Provider availability and context limit from the host.
    /// * `workspace_status` - Workspace state from the host.
    /// * `issues` - Existing probe issues, in their original order.
    ///
    /// # Returns
    /// A report containing all issues and the token estimate.
    ///
    /// # Examples
    /// See [ValidationPolicy] for construction; pass host observations to this method.
    pub fn validate(
        &self,
        subtasks: &[SubTask],
        provider_status: ProviderStatus,
        workspace_status: WorkspaceStatus,
        mut issues: Vec<ValidationIssue>,
    ) -> ValidationReport {
        self.validate_configuration(&mut issues);
        self.validate_dependencies(subtasks, &mut issues);
        let estimated_tokens = self.estimate_token_usage(subtasks, &provider_status, &mut issues);
        let is_valid = !issues.iter().any(|i| i.severity == IssueSeverity::Error);
        ValidationReport {
            is_valid,
            issues,
            estimated_tokens,
            provider_status,
            workspace_status,
        }
    }
}
