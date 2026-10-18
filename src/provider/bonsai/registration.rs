//! Separate `bonsai` provider registration, with Vault precedence and explicit local opt-in.
use super::{BonsaiConfig, BonsaiProvider};
use crate::provider::ProviderRegistry;
use std::sync::Arc;
pub(crate) fn environment(registry: &mut ProviderRegistry) {
    if registry.get("bonsai").is_some() {
        return;
    }
    if std::env::var_os("BONSAI_MODEL_PATH").is_none()
        && std::env::var("CODETETHER_BONSAI").ok().as_deref() != Some("1")
    {
        return;
    }
    match BonsaiConfig::from_environment().and_then(BonsaiProvider::new) {
        Ok(provider) => registry.register(Arc::new(provider)),
        Err(error) => tracing::warn!(provider="bonsai", %error, "Native provider unavailable"),
    }
}
pub(crate) use codetether_provider_factory::bonsai::vault;
