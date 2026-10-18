//! Box the CUDA provider using its selected inference runtime.
use super::{LocalProvider, Provider};
use crate::{LocalCudaProvider, Runtime};
use std::sync::Arc;

impl<R: Runtime> LocalProvider for LocalCudaProvider<R> {
    fn create(model: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(LocalCudaProvider::<R>::new(model)?))
    }
    fn create_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(LocalCudaProvider::<R>::with_paths(
            model,
            path,
            tokenizer,
            architecture,
        )?))
    }
}
