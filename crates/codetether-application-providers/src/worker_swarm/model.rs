//! Resolve worker swarm models without eagerly loading a provider registry.
use crate::{Application, ProviderRegistry};
/// Resolve an explicit model or choose a provider and model for the given tier.
/// # Arguments
/// `explicit_model` wins when nonblank; `model_tier` selects fallback preferences.
/// # Returns
/// A model reference, or `None` when provider loading or availability fails.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>() {
/// let model = codetether_application_providers::worker_swarm::resolve_model::<H>(
///     Some("provider/model".into()), None).await;
/// assert_eq!(model.as_deref(), Some("provider/model"));
/// # }
/// ```
pub async fn resolve_model<H: Application>(
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> Option<String> {
    codetether_a2a_worker_core::execution_policy::model_selection::resolve_swarm_model(
        explicit_model,
        model_tier,
        ProviderRegistry::<H>::shared_from_vault,
    )
    .await
}
