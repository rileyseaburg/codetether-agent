use anyhow::Result;
use async_trait::async_trait;
/// Host-provided text completion used for initial and repair decomposition requests.
///
/// # Implementors
/// Adapters select providers, configure model/temperature and return response text.
/// The shared planner owns validation and the single permitted repair attempt.
#[async_trait]
pub trait PlanCompletion: Send + Sync {
    /// Complete one prompt using the host's fixed request configuration.
    ///
    /// # Errors
    /// Returns an error when the provider request fails.
    async fn complete(&self, prompt: String) -> Result<String>;
}
