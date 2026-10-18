//! Construct shared providers while forwarding only local runtime operations.
use super::{ApplicationHost, Arc, Identity, LocalProvider, Provider};
use codetether_provider_bedrock::provider as bedrock;
use codetether_provider_copilot::application::CopilotProvider;
impl<I: Identity, L: LocalProvider> crate::Host for ApplicationHost<I, L> {
    async fn ensure_bedrock_fresh() {
        let _ = bedrock::sso_refresh::ensure_fresh().await;
    }
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
        L::create(model)
    }
    fn local_cuda_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        L::create_with_paths(model, path, tokenizer, architecture)
    }
    fn copilot_new(key: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(CopilotProvider::<I>::new(key)?))
    }
    fn copilot_with_base_url(
        key: String,
        url: String,
        name: &str,
    ) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(CopilotProvider::<I>::with_base_url(
            key, url, name,
        )?))
    }
    fn copilot_enterprise(key: String, url: String) -> anyhow::Result<Arc<dyn Provider>> {
        Ok(Arc::new(CopilotProvider::<I>::enterprise(key, url)?))
    }
}
