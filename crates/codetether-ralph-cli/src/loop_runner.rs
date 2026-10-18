//! Loop execution contract for composing provider-backed Ralph commands.
use codetether_provider_core::Provider;
use codetether_ralph_core::RalphConfig;
use std::{path::PathBuf, sync::Arc};

/// Execute a Ralph loop and present the requested report.
/// # Implementors
/// Runtime loop types implement this contract so provider initialization can
/// compose them without depending on a specific runtime implementation.
#[async_trait::async_trait(?Send)]
pub trait LoopRunner {
    /// Load the PRD, execute its loop and print the result.
    /// # Arguments
    /// Receives the PRD path, selected provider/model, runtime settings and
    /// whether to print JSON instead of the text report.
    /// # Returns
    /// Unit after execution and output finish.
    /// # Errors
    /// Returns loop construction, execution or report serialization errors.
    /// # Examples
    /// ```no_run
    /// # async fn example<L: codetether_ralph_cli::LoopRunner>(
    /// # provider: std::sync::Arc<dyn codetether_provider_core::Provider>) -> anyhow::Result<()> {
    /// L::execute("prd.json".into(), provider, "model".into(), Default::default(), false).await
    /// # }
    /// ```
    async fn execute(
        prd: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        json: bool,
    ) -> anyhow::Result<()>;
}
