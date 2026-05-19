use super::{IssueSeverity, ValidationReport};
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
        self.validate_configuration(&mut issues);
        self.validate_dependencies(subtasks, &mut issues);
        let estimated_tokens = self.estimate_token_usage(subtasks, &provider_status, &mut issues);
        let is_valid = !issues.iter().any(|i| i.severity == IssueSeverity::Error);
        Ok(ValidationReport {
            is_valid,
            issues,
            estimated_tokens,
            provider_status,
            workspace_status,
        })
    }
}
