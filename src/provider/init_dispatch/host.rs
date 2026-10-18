//! Bind shared application constructors to identity and local CUDA runtime.
use crate::provider::{Provider, copilot, local_cuda};
use std::sync::Arc;
pub(in crate::provider) type Native =
    codetether_provider_factory::application::ApplicationHost<copilot::ApplicationIdentity, Local>;
/// Local provider construction using the application's feature-selected runtime.
pub(in crate::provider) struct Local;
impl codetether_provider_factory::application::LocalProvider for Local {
    fn create(model: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(local_cuda::LocalCudaProvider::new(model)?))
    }
    fn create_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(local_cuda::LocalCudaProvider::with_paths(
            model,
            path,
            tokenizer,
            architecture,
        )?))
    }
}
