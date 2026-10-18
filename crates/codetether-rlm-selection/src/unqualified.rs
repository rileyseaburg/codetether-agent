//! Unqualified models prefer configured local CUDA, then OpenRouter.
use crate::{ProviderLookup, SelectedProvider};
pub(super) fn select(
    model_ref: &str,
    lookup: &ProviderLookup<'_>,
) -> anyhow::Result<SelectedProvider> {
    // Unqualified model: local_cuda first when usable, then openrouter.
    if crate::environment::local_cuda_runtime_configured() {
        if let Some(provider) = lookup("local_cuda") {
            return Ok((provider, model_ref.to_string(), "local_cuda".to_string()));
        }
        anyhow::bail!(
            "Local CUDA is configured but unavailable. \
                 Fix local CUDA setup (CUDA-enabled binary + valid LOCAL_CUDA_MODEL_PATH and \
                 LOCAL_CUDA_TOKENIZER_PATH) or explicitly select OpenRouter with \
                 --model openrouter/<model>."
        );
    }
    if let Some(provider) = lookup("openrouter") {
        return Ok((provider, model_ref.to_string(), "openrouter".to_string()));
    }
    anyhow::bail!(
        "No provider available for model '{}'. Configure local_cuda or openrouter.",
        model_ref
    );
}
