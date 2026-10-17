//! Compatibility exports for the shared Codex provider.
//! Use [OpenAiCodexProvider] for API-key or opted-in OAuth-backed completions.
pub use codetether_provider_codex::{
    ChatGptBackendAuth, OAuthCredentials, OpenAiCodexProvider, OpenAiRealtimeConnection,
    reasoning_catalog, runtime_config,
};
pub(crate) use codetether_provider_codex::{model_catalog, model_discovery, service_tier_catalog};
