use super::{IssueCategory, IssueSeverity, ProviderStatus, SwarmValidator, ValidationIssue};
use crate::provider::ProviderRegistry;
use anyhow::Result;

impl SwarmValidator {
    pub(super) async fn validate_provider(
        &self,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<ProviderStatus> {
        let registry = match ProviderRegistry::from_vault().await {
            Ok(registry) => registry,
            Err(error) => return Ok(self.provider_registry_error(issues, error)),
        };
        let available_providers = registry.list();
        let is_available = available_providers.contains(&self.provider.as_str());
        if !is_available {
            issues.push(self.unavailable_provider_issue(&available_providers));
        }
        Ok(ProviderStatus {
            provider: self.provider.clone(),
            model: self.model.clone(),
            is_available,
            has_credentials: is_available,
            context_window: self.context_window(&registry).await,
        })
    }

    fn provider_registry_error(
        &self,
        issues: &mut Vec<ValidationIssue>,
        error: anyhow::Error,
    ) -> ProviderStatus {
        issues.push(ValidationIssue {
            severity: IssueSeverity::Error,
            category: IssueCategory::Provider,
            message: format!("Failed to load provider registry: {error}"),
            suggestion: Some("Check VAULT_ADDR and VAULT_TOKEN environment variables".into()),
        });
        ProviderStatus {
            provider: self.provider.clone(),
            model: self.model.clone(),
            is_available: false,
            has_credentials: false,
            context_window: 0,
        }
    }
}
