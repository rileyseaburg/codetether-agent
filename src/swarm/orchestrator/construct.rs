use super::*;
impl Orchestrator {
    /// Create a new orchestrator
    pub async fn new(config: SwarmConfig) -> Result<Self> {
        let providers = ProviderRegistry::from_vault().await?;
        let provider_list = providers.list();

        if provider_list.is_empty() {
            anyhow::bail!("No providers available for orchestration");
        }

        let (provider, model) = selection::select_model(&config, provider_list.as_slice())?;

        tracing::info!("Orchestrator using model {} via {}", model, provider);

        Ok(Self {
            config,
            providers,
            state: Default::default(),
            model,
            provider,
        })
    }
}
