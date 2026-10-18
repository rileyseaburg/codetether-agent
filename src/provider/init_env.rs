//! Bind shared environment registration to the instrumented native registry.
use super::ProviderRegistry;
/// Register missing providers from environment variables in shared policy order.
pub fn register_env_fallbacks(registry: &mut ProviderRegistry) {
    codetether_provider_factory::environment::register_env_fallbacks::<
        super::init_dispatch::host::Native,
    >(registry);
}
