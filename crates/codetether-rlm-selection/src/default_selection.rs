//! Default selection when no model argument is supplied.
use crate::{ProviderLookup, SelectedProvider};
pub(super) fn select(lookup: &ProviderLookup<'_>) -> anyhow::Result<SelectedProvider> {
    // No explicit model: prefer local_cuda when usable, otherwise openrouter.
    if crate::environment::local_cuda_runtime_configured() {
        if let Some(provider) = lookup("local_cuda") {
            return Ok((
                provider,
                crate::environment::local_cuda_model_name(),
                "local_cuda".to_string(),
            ));
        }
        anyhow::bail!(
            "Local CUDA is configured but unavailable. \
             Fix local CUDA setup (CUDA-enabled binary + valid LOCAL_CUDA_MODEL_PATH and \
             LOCAL_CUDA_TOKENIZER_PATH) or disable local CUDA env vars to use OpenRouter fallback."
        );
    }

    if let Some(provider) = lookup("openrouter") {
        return Ok((
            provider,
            crate::environment::default_openrouter_rlm_model(),
            "openrouter".to_string(),
        ));
    }

    anyhow::bail!(
        "No usable RLM provider found. Configure local CUDA \
         (LOCAL_CUDA_MODEL_PATH + LOCAL_CUDA_TOKENIZER_PATH) \
         or OpenRouter credentials."
    );
}
