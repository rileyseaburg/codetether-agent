//! Application-owned provider constructors.
use crate::provider::{Provider, bedrock, copilot, local_cuda};
use std::sync::Arc;
pub(super) struct Native;
impl codetether_provider_factory::Host for Native {
    fn bedrock_with_credentials(
        credentials: bedrock::AwsCredentials,
        region: String,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(bedrock::BedrockProvider::with_credentials(
            credentials,
            region,
        )?))
    }
    fn bedrock_with_region(key: String, region: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(bedrock::BedrockProvider::with_region(
            key, region,
        )?))
    }
    fn local_cuda_new(model: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(local_cuda::LocalCudaProvider::new(model)?))
    }
    fn local_cuda_with_paths(
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
    fn copilot_new(key: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(copilot::CopilotProvider::new(key)?))
    }
    fn copilot_with_base_url(
        key: String,
        url: String,
        name: &str,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(copilot::CopilotProvider::with_base_url(
            key, url, name,
        )?))
    }
    fn copilot_enterprise(key: String, url: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(copilot::CopilotProvider::enterprise(key, url)?))
    }
}
