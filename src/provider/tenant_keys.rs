//! Native compatibility wrapper for shared task credential ownership.
use super::registry::ProviderRegistry;
pub use codetether_secrets::TenantProviderKeyPayload;
/// Task-owned keys using the native provider factory and registry.
pub type PerTaskProviderKeys = codetether_provider_registry::BoundTaskKeys<
    codetether_application_providers::task_factory::TaskFactory<
        super::registry::application::Native,
    >,
    ProviderRegistry,
>;
