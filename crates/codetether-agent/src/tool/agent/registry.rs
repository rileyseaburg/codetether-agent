//! Shared native agent-tool provider cache.
pub(super) use codetether_native_agent::provider_cache::get_registry;
#[cfg(test)]
pub(super) use codetether_native_agent::provider_cache::set_registry_for_test;
