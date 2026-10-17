use super::ValidationReport;
use crate::swarm::{SubTask, SwarmConfig};
use anyhow::Result;

/// Pre-flight validator for swarm execution.
pub struct SwarmValidator {
    pub(super) config: SwarmConfig,
    pub(super) provider: String,
    pub(super) model: String,
}

impl SwarmValidator {
    /// Create a new validator.
    pub fn new(config: SwarmConfig, provider: String, model: String) -> Self {
        Self {
            config,
            provider,
            model,
        }
    }

    /// Run all validation checks.
    pub async fn validate(&self, subtasks: &[SubTask]) -> Result<ValidationReport> {
        let mut issues = Vec::new();
        let provider_status = self.validate_provider(&mut issues).await?;
        let workspace_status = self.validate_workspace(&mut issues)?;
        Ok(
            codetether_swarm_core::validation::ValidationPolicy::new(self.config.clone()).validate(
                subtasks,
                provider_status,
                workspace_status,
                issues,
            ),
        )
    }
}
