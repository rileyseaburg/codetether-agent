use super::*;
impl<H: Host> Orchestrator<H> {
    /// Create an orchestrator with an initialized provider registry.
    /// # Arguments
    /// `config` supplies model selection, parallelism and sub-agent limits.
    /// # Returns
    /// An empty coordinator retaining its provider registry and selected model.
    /// # Errors
    /// Returns initialization errors, missing-provider errors or an unavailable explicit model provider.
    /// # Examples
    /// ```no_run
    /// async fn build<H: codetether_swarm_orchestrator::Host>() -> anyhow::Result<()> {
    ///     let planner = codetether_swarm_orchestrator::Orchestrator::<H>::new(Default::default()).await?;
    ///     assert!(planner.all_subtasks().is_empty());
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: SwarmConfig) -> Result<Self> {
        let providers = H::load().await?;
        let provider_list = H::list(&providers);

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
