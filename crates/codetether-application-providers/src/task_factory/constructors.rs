//! Forward provider creation unchanged to the application's selected factory.
use super::TaskFactory;
use crate::Application;
use codetether_bedrock_auth::AwsCredentials;
use codetether_provider_core::Provider;
use codetether_provider_factory::Host;
use std::sync::Arc;
impl<H: Application> Host for TaskFactory<H> {
    async fn ensure_bedrock_fresh() {
        H::Factory::ensure_bedrock_fresh().await;
    }
    fn bedrock_with_credentials(
        credentials: AwsCredentials,
        region: String,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::bedrock_with_credentials(credentials, region)
    }
    fn bedrock_with_region(key: String, region: String) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::bedrock_with_region(key, region)
    }
    fn local_cuda_new(model: String) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::local_cuda_new(model)
    }
    fn local_cuda_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::local_cuda_with_paths(model, path, tokenizer, architecture)
    }
    fn copilot_new(key: String) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::copilot_new(key)
    }
    fn copilot_with_base_url(
        key: String,
        url: String,
        name: &str,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::copilot_with_base_url(key, url, name)
    }
    fn copilot_enterprise(key: String, url: String) -> anyhow::Result<Arc<dyn Provider>> {
        H::Factory::copilot_enterprise(key, url)
    }
}
