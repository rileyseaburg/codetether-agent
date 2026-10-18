//! Native compatibility wrapper for shared task credential ownership.
use super::registry::ProviderRegistry;
pub use codetether_secrets::TenantProviderKeyPayload;
mod platform;
/// Task-owned keys using the native provider factory and registry.
pub type PerTaskProviderKeys = codetether_provider_registry::BoundTaskKeys<
    super::init_dispatch::host::Native,
    ProviderRegistry,
>;
