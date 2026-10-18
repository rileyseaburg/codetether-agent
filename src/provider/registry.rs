//! Provider registry using the native factory and process-wide cache.
mod application;
/// Application provider registry with native initialization and runtime bindings.
pub type ProviderRegistry = codetether_application_providers::ProviderRegistry<application::Native>;
