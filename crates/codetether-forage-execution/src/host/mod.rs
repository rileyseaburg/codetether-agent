//! Application execution and authorization contracts.
use anyhow::Result;
use codetether_cli_options::RunArgs;
use codetether_go_ralph::GoRalphResult;
use codetether_okr::{Okr, OkrRun};
use codetether_provider_core::Provider;
use codetether_swarm_core::SwarmConfig;
use codetether_tool_core::ToolResult;
use std::{future::Future, sync::Arc};
mod providers;
mod swarm;
pub use providers::Providers;
pub use swarm::Swarm;
/// Application services used by forage execution.
/// # Implementors
/// Adapters bind CLI execution, provider initialization, swarm construction and
/// tool authorization while shared code owns engine policy and reporting.
pub trait Host {
    /// Retained provider registry for the lifetime of Go execution.
    type Registry: Providers + Send;
    /// Swarm executor retained through its bounded execution.
    type Swarm: Swarm + Send + Sync;
    /// Execute prepared run options.
    fn run(args: RunArgs) -> impl Future<Output = Result<()>> + Send;
    /// Construct a swarm executor with the prepared configuration.
    fn swarm(config: SwarmConfig) -> Self::Swarm;
    /// Load application providers from the configured credential sources.
    fn providers() -> impl Future<Output = Result<Self::Registry>> + Send;
    /// Execute Go with the selected model and shared iteration/concurrency limits.
    fn go(
        task: &str,
        okr: &mut Okr,
        run: &mut OkrRun,
        provider: Arc<dyn Provider>,
        model: &str,
        max_iterations: usize,
        max_concurrent: usize,
    ) -> impl Future<Output = Result<GoRalphResult>> + Send;
    /// Return a blocked-tool result or allow the supplied invocation.
    fn evaluate_tool_invocation(
        name: &str,
        args: &serde_json::Value,
    ) -> impl Future<Output = Option<ToolResult>> + Send;
}
