//! Native loop construction and execution used by interactive commands.
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, events::RalphEvent};
use std::{future::Future, path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
/// Native execution boundary for background Ralph runs.
/// # Implementors
/// Applications construct their existing Ralph loop and attach the event sender.
pub trait Host: Send + 'static {
    /// Native loop instance retained until background execution finishes.
    type Run: Send;
    /// Construct the native loop and attach its event sender.
    /// # Errors
    /// Returns the native loop initialization failure.
    fn create(
        path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        events: mpsc::Sender<RalphEvent>,
    ) -> impl Future<Output = anyhow::Result<Self::Run>> + Send;
    /// Run the initialized loop to completion.
    /// # Errors
    /// Returns the native loop execution failure.
    fn run(run: &mut Self::Run) -> impl Future<Output = anyhow::Result<()>> + Send;
}
