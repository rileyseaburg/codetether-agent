use super::ValidationReport;
use anyhow::Result;
use codetether_swarm_core::{SubTask, SwarmConfig};

/// Pre-flight validator for swarm execution using application provider loading.
///
/// # Examples
/// ```no_run
/// # fn example<H: codetether_swarm_validation::Host>(config: codetether_swarm_core::SwarmConfig) {
/// let _validator = codetether_swarm_validation::SwarmValidator::<H>::new(
///     config, "openai".into(), "model".into(),
/// );
/// # }
/// ```
pub struct SwarmValidator<H: crate::Host> {
    host: std::marker::PhantomData<fn() -> H>,
    pub(super) config: SwarmConfig,
    pub(super) provider: String,
    pub(super) model: String,
}

impl<H: crate::Host> SwarmValidator<H> {
    /// Create a new validator.
    ///
    /// # Arguments
    /// * `config` — Swarm execution configuration to validate.
    /// * `provider` — Provider registry key used by the swarm.
    /// * `model` — Model identifier used for context-window discovery.
    /// # Returns
    /// A validator that loads providers through `H` when validation runs.
    /// # Examples
    /// See the construction example on [`SwarmValidator`].
    pub fn new(config: SwarmConfig, provider: String, model: String) -> Self {
        Self {
            host: std::marker::PhantomData,
            config,
            provider,
            model,
        }
    }

    /// Run all validation checks.
    ///
    /// # Arguments
    /// * `subtasks` — Proposed swarm tasks to check before execution.
    /// # Returns
    /// The ordered validation report with provider and workspace observations.
    /// # Errors
    /// Returns an error if the current working directory cannot be resolved.
    /// Provider initialization failures become issues in the returned report.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_swarm_validation::Host>(validator: &codetether_swarm_validation::SwarmValidator<H>, tasks: &[codetether_swarm_core::SubTask]) -> anyhow::Result<()> {
    /// let _report = validator.validate(tasks).await?;
    /// # Ok(()) }
    /// ```
    pub async fn validate(&self, subtasks: &[SubTask]) -> Result<ValidationReport> {
        let mut issues = Vec::new();
        let provider_status = self.validate_provider(&mut issues).await?;
        let workspace_status = self.validate_workspace(&mut issues)?;
        Ok(
            codetether_swarm_core::validation::ValidationPolicy::new(self.config.clone()).validate(
                subtasks,
                provider_status,
                workspace_status,
                issues,
            ),
        )
    }
}
