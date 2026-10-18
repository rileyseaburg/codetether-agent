//! Vault configuration of the shared Bonsai provider.
use codetether_provider_bonsai::{BonsaiConfig, BonsaiProvider};
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::{path::PathBuf, sync::Arc};

/// Construct Bonsai from Vault model paths and the configured CUDA ordinal.
///
/// # Arguments
/// * `secrets` — Model and tokenizer paths plus an optional CUDA ordinal.
/// # Returns
/// A provider, or `None` if required fields or construction are invalid.
pub fn vault(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    let model_path = PathBuf::from(secrets.extra.get("model_path")?.as_str()?);
    let tokenizer_path = PathBuf::from(secrets.extra.get("tokenizer_path")?.as_str()?);
    let ordinal = secrets
        .extra
        .get("cuda_ordinal")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let cuda_ordinal = usize::try_from(ordinal).ok()?;
    match BonsaiProvider::new(BonsaiConfig {
        model_path,
        tokenizer_path,
        cuda_ordinal,
    }) {
        Ok(provider) => Some(Arc::new(provider)),
        Err(error) => {
            tracing::warn!(provider="bonsai", %error, "Native provider unavailable");
            None
        }
    }
}
