//! Explicitly qualified provider selection.
use crate::{ProviderLookup, SelectedProvider};
pub(super) fn select(
    provider_name: &str,
    model_name: &str,
    lookup: &ProviderLookup<'_>,
) -> anyhow::Result<SelectedProvider> {
    let normalized = crate::aliases::normalize_provider_alias(provider_name);
    if normalized == "local_cuda" {
        let provider = lookup("local_cuda").ok_or_else(|| {
            anyhow::anyhow!(
                "local_cuda selected explicitly but provider is unavailable. \
                         Build/install with CUDA support \
                         (--features candle-cuda) and set \
                         LOCAL_CUDA_MODEL_PATH + LOCAL_CUDA_TOKENIZER_PATH."
            )
        })?;
        if !crate::environment::local_cuda_runtime_configured() {
            anyhow::bail!(
                "local_cuda selected explicitly but runtime is not configured. \
                         Set LOCAL_CUDA_MODEL_PATH and LOCAL_CUDA_TOKENIZER_PATH \
                         (or CODETETHER_LOCAL_CUDA_MODEL_PATH / CODETETHER_LOCAL_CUDA_TOKENIZER_PATH)."
            );
        }
        return Ok((provider, model_name.to_string(), "local_cuda".to_string()));
    }

    let provider = lookup(normalized).ok_or_else(|| {
        anyhow::anyhow!(
            "Requested provider '{}' is not available. Configure it in Vault or env.",
            normalized
        )
    })?;
    return Ok((provider, model_name.to_string(), normalized.to_string()));
}
