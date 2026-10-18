//! Preserve the disabled provider's construction error in boxed factories.
use super::{LocalProvider, Provider};
use crate::LocalCudaProvider;
use std::sync::Arc;

impl LocalProvider for LocalCudaProvider {
    fn create(model: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(LocalCudaProvider::new(model)?))
    }
    fn create_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(LocalCudaProvider::with_paths(
            model,
            path,
            tokenizer,
            architecture,
        )?))
    }
}
