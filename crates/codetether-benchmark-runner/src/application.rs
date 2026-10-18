//! Application operations used by the benchmark suite integration.
use anyhow::Result;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, RalphState};
use std::{future::Future, path::PathBuf, sync::Arc};
/// Provider and Ralph operations supplied by the embedding application.
/// # Implementors
/// Applications choose their registry and loop types and preserve native setup.
pub trait Application: Send + Sync + 'static {
    /// Provider state retained throughout all PRDs for a model.
    type Registry: Send + Sync;
    /// Ralph loop retained until the benchmark result is constructed.
    type Loop: Send;
    /// Application version included in the suite report.
    fn version() -> &'static str;
    /// Load the registry for one configured benchmark model.
    /// # Errors
    /// Returns provider initialization failures.
    fn registry() -> impl Future<Output = Result<Self::Registry>> + Send;
    /// Find a provider without changing registry ownership.
    fn provider(registry: &Self::Registry, name: &str) -> Option<Arc<dyn Provider>>;
    /// Construct the loop inside the prepared benchmark workspace.
    /// # Errors
    /// Returns PRD loading or loop construction failures.
    fn start(
        path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> impl Future<Output = Result<Self::Loop>> + Send;
    /// Execute the loop while preserving its lifetime.
    /// # Errors
    /// Returns execution failures.
    fn run(loop_state: &mut Self::Loop) -> impl Future<Output = Result<RalphState>> + Send;
}
