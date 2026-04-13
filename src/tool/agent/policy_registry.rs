//! Registry-backed model cost checks for spawn policy.
//!
//! This module evaluates provider model metadata when allowlists alone
//! do not determine eligibility.
//!
//! # Examples
//!
//! ```ignore
//! let free = registry_model_is_free("provider", "model", &registry).await;
//! ```

use crate::provider::{ModelInfo, ProviderRegistry};

/// Checks provider metadata to see whether a model is effectively free.
///
/// # Examples
///
/// ```ignore
/// let free = registry_model_is_free("provider", "model", &registry).await;
/// ```
pub(super) async fn registry_model_is_free(
    provider: &str,
    model_id: &str,
    registry: &ProviderRegistry,
) -> bool {
    let Some(entry) = registry.get(provider) else {
        return false;
    };
    match entry.list_models().await {
        Ok(models) => models
            .into_iter()
            .any(|model| model_is_free(&model, model_id)),
        Err(_) => false,
    }
}

fn model_is_free(model: &ModelInfo, model_id: &str) -> bool {
    model.id.eq_ignore_ascii_case(model_id)
        && model.input_cost_per_million.unwrap_or(1.0) <= 0.0
        && model.output_cost_per_million.unwrap_or(1.0) <= 0.0
}
