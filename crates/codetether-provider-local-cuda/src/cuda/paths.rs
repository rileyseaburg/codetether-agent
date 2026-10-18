//! Local CUDA paths operations.
use super::*;
impl<R: Runtime> LocalCudaProvider<R> {
    pub(super) fn resolve_model_path(&self) -> Option<String> {
        self.model_cache
            .as_ref()
            .map(|c| c.model_path.clone())
            .or_else(|| first_env(&["LOCAL_CUDA_MODEL_PATH", "CODETETHER_LOCAL_CUDA_MODEL_PATH"]))
    }

    pub(super) fn resolve_tokenizer_path(&self) -> Option<String> {
        self.model_cache
            .as_ref()
            .and_then(|c| c.tokenizer_path.clone())
            .or_else(|| {
                first_env(&[
                    "LOCAL_CUDA_TOKENIZER_PATH",
                    "CODETETHER_LOCAL_CUDA_TOKENIZER_PATH",
                ])
            })
    }

    pub(super) fn resolve_architecture(&self) -> Option<String> {
        self.model_cache
            .as_ref()
            .and_then(|c| c.architecture.clone())
            .or_else(|| codetether_provider_core::local::catalog::arch_or_env(&self.model_name))
    }

    pub(super) fn resolve_device_preference(&self) -> CandleDevicePreference {
        if let Some(v) = first_env(&["LOCAL_CUDA_DEVICE", "CODETETHER_LOCAL_CUDA_DEVICE"]) {
            return CandleDevicePreference::from_env(&v);
        }
        CandleDevicePreference::Auto
    }
}
