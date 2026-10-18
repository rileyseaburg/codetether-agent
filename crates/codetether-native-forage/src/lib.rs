//! Native forage scoring, clarification, providers and authorization.
//!
//! [`Host`] implements shared forage services while [`Execution`] supplies the
//! application's CLI, Go and swarm execution entry points. Optional TetherScript
//! scoring uses the same native plugin and policy evaluator as tool execution.
use codetether_cli_options::RunArgs;
use codetether_go_ralph::GoRalphResult;
use codetether_okr::{Okr, OkrRun};
use codetether_provider_core::Provider;
use codetether_swarm_core::SwarmConfig;
use std::{future::Future, marker::PhantomData, sync::Arc};
mod runtime;
mod scoring;
mod services;

/// Application execution operations needed by native forage services.
/// # Implementors
/// Applications preserve their CLI runner, Go loop and concrete swarm executor.
pub trait Execution {
    /// Swarm executor retained through bounded execution.
    type Swarm: codetether_forage_execution::Swarm + Send + Sync;
    /// Execute prepared CLI options.
    fn run(args: RunArgs) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Construct the concrete swarm executor using the supplied configuration.
    fn swarm(config: SwarmConfig) -> Self::Swarm;
    /// Execute Go with the shared iteration and concurrency limits.
    fn go(
        task: &str,
        okr: &mut Okr,
        run: &mut OkrRun,
        provider: Arc<dyn Provider>,
        model: &str,
        max_iterations: usize,
        max_concurrent: usize,
    ) -> impl Future<Output = anyhow::Result<GoRalphResult>> + Send;
}

/// Shared forage host using native services and application execution `E`.
/// # Examples
/// ```no_run
/// # async fn example<E: codetether_native_forage::Execution>(args: codetether_cli_options::ForageArgs) -> anyhow::Result<()> {
/// let _summary = codetether_forage_runtime::execute_with_summary::<codetether_native_forage::Host<E>>(args).await?;
/// # Ok(()) }
/// ```
pub struct Host<E>(PhantomData<fn() -> E>);

#[cfg(test)]
mod tests;
