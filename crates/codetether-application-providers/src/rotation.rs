//! Application registry adapter for shared auto-chat model selection.
use crate::{Application, Provider, ProviderRegistry};
use codetether_autochat_core::model_rotation;
pub use codetether_autochat_core::model_rotation::RelayModelRotation;
use std::sync::Arc;
/// Build rotation using the existing initialized application registry.
/// # Arguments
/// `registry` supplies providers; `requested_model_ref` identifies the preferred model.
/// # Returns
/// The same ordered rotation produced by the shared auto-chat policy.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>(registry: std::sync::Arc<codetether_application_providers::ProviderRegistry<H>>) {
/// let _rotation = codetether_application_providers::rotation::build_round_robin_model_rotation(&registry, "provider/model").await;
/// # }
/// ```
pub async fn build_round_robin_model_rotation<H: Application>(
    registry: &Arc<ProviderRegistry<H>>,
    requested_model_ref: &str,
) -> RelayModelRotation {
    model_rotation::build_round_robin_model_rotation(registry.as_ref(), requested_model_ref).await
}
/// Resolve a provider handle using the shared selection policy.
/// # Arguments
/// `registry` supplies providers; `model_ref` is the requested provider/model selector.
/// # Returns
/// The selected provider and local model name, or `None` if unavailable.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>(registry: std::sync::Arc<codetether_application_providers::ProviderRegistry<H>>) {
/// let _provider = codetether_application_providers::rotation::resolve_provider_for_model_autochat(&registry, "provider/model");
/// # }
/// ```
pub fn resolve_provider_for_model_autochat<H: Application>(
    registry: &Arc<ProviderRegistry<H>>,
    model_ref: &str,
) -> Option<(Arc<dyn Provider>, String)> {
    model_rotation::resolve_provider_for_model_autochat(registry.as_ref(), model_ref)
}
