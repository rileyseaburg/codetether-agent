use super::{IssueCategory, IssueSeverity, SwarmValidator, ValidationIssue};
use crate::provider::ProviderRegistry;

impl SwarmValidator {
    pub(super) async fn context_window(&self, registry: &ProviderRegistry) -> usize {
        let Some(provider) = registry.get(&self.provider) else {
            return 128_000;
        };
        provider
            .list_models()
            .await
            .ok()
            .and_then(|models| model_context_window(&models, &self.model))
            .unwrap_or(128_000)
    }

    pub(super) fn unavailable_provider_issue(&self, available: &[&str]) -> ValidationIssue {
        ValidationIssue {
            severity: IssueSeverity::Error,
            category: IssueCategory::Provider,
            message: format!(
                "Provider '{}' is not available. Available providers: {}",
                self.provider,
                available.join(", ")
            ),
            suggestion: Some(format!(
                "Set credentials in Vault at secret/codetether/providers/{}",
                self.provider
            )),
        }
    }
}

fn model_context_window(models: &[crate::provider::ModelInfo], model_id: &str) -> Option<usize> {
    models
        .iter()
        .find(|model| model.id == model_id)
        .map(|model| model.context_window)
}
