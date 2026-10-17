//! Construct initial state and retain application runtime dependencies.
use super::*;
impl RalphLoop {
    /// Create a new Ralph loop
    ///
    /// # Arguments
    /// * `prd_path` - PRD file to load and resume.
    /// * `provider` - Provider retained by the loop.
    /// * `model` - Model identifier for execution.
    /// * `config` - Runtime limits and integrations.
    /// # Returns
    /// A pending loop initialized from the PRD's completed stories.
    /// # Errors
    /// Returns PRD loading or current-directory resolution errors.
    /// # Examples
    /// ```no_run
    /// # async fn example(provider: std::sync::Arc<dyn codetether_agent::provider::Provider>) -> anyhow::Result<()> {
    /// let runner = codetether_agent::ralph::RalphLoop::new("prd.json".into(), provider, "model".into(), Default::default()).await?;
    /// assert_eq!(runner.status().status, codetether_agent::ralph::RalphStatus::Pending);
    /// # Ok(()) }
    /// ```
    pub async fn new(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> anyhow::Result<Self> {
        let state = codetether_ralph_core::lifecycle::load_initial_state(
            &prd_path,
            config.max_iterations,
            |prd| {
                info!(
                    "Loaded PRD: {} - {} ({} stories, {} already passed)",
                    prd.project,
                    prd.feature,
                    prd.user_stories.len(),
                    prd.passed_count()
                );
            },
        )
        .await?;

        Ok(Self {
            state,
            provider,
            model,
            config,
            event_tx: None,
            bus: None,
            registry: None,
            store: None,
            delegation: None,
            run_id: uuid::Uuid::new_v4().to_string(),
        })
    }
}
