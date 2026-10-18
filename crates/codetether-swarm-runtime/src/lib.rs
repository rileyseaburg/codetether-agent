//! # Swarm Run Lifecycle
//!
//! Decomposes one user task, executes dependency stages, and finalizes the
//! parent-facing [`SwarmResult`](codetether_swarm_core::SwarmResult). `Run` carries the
//! orchestrator, stage results, telemetry identity, and dependency state.
//!
//! The executor enters through `execute`; child modules own preparation,
//! ordered stage traversal, result publication, statistics, and finalization.

mod aggregate;
mod control;
mod decomposed_event;
mod empty;
mod final_stats;
mod finish;
mod prepare;
mod prepare_plan;
mod stage_input;
mod stage_operations;
mod stage_runtime;
mod stage_stats;
mod stages;
mod state;
mod store;

mod host;
mod stage_dispatch;
use anyhow::Result;
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};
pub use host::{Host, Services, Stages};

/// Execute a complete swarm run under the supplied executor services.
/// # Arguments
/// Receives native services, original task text and decomposition strategy.
/// # Returns
/// The finalized parent-facing swarm result.
/// # Errors
/// Returns planning, readiness or stage execution errors.
/// # Examples
/// ```no_run
/// async fn run<H: codetether_swarm_runtime::Host>(host: &H) -> anyhow::Result<()> {
///     codetether_swarm_runtime::execute(host, "inspect", codetether_swarm_core::DecompositionStrategy::None).await?;
///     Ok(())
/// }
/// ```
pub async fn execute<H: Host>(
    executor: &H,
    task: &str,
    strategy: DecompositionStrategy,
) -> Result<SwarmResult> {
    match prepare::run(executor, task, strategy).await? {
        prepare::Prepared::Empty(result) => Ok(result),
        prepare::Prepared::Ready(mut run) => {
            stages::execute(&mut run).await?;
            finish::execute(run).await
        }
    }
}

/// Inline worker execution and lifecycle event forwarding.
pub mod inline;

mod executor_cache;
mod executor_state;
pub use executor_state::ExecutorState;

mod executor_access;
mod executor_configure;

mod application_executor;
pub use application_executor::{ExecutorHost, SwarmExecutor};

pub use application_executor::kubernetes;

pub use application_executor::local as local_stage;
pub use application_executor::local::LocalStageHost;
