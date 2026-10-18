//! Swarm model resolution helpers.
//!
//! This module picks an execution model for swarm tasks when the queue payload
//! does not explicitly provide one.
//!
//! # Examples
//!
//! ```ignore
//! let model = resolve_swarm_model(None, Some("fast")).await;
//! ```

use crate::provider::ProviderRegistry;

/// Resolves the effective swarm model from explicit or inferred settings.
///
/// Explicit models win. Otherwise the worker loads the provider registry and
/// chooses a preferred provider for the requested tier.
///
/// # Examples
///
/// ```ignore
/// let model = resolve_swarm_model(Some("zai/glm-5.1".to_string()), None).await;
/// assert_eq!(model.as_deref(), Some("zai/glm-5.1"));
/// ```
pub(super) async fn resolve_swarm_model(
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> Option<String> {
    codetether_a2a_worker_core::execution_policy::model_selection::resolve_swarm_model(
        explicit_model,
        model_tier,
        ProviderRegistry::shared_from_vault,
    )
    .await
}
