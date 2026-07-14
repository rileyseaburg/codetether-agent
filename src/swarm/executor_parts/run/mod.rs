//! # Swarm Run Lifecycle
//!
//! Decomposes one user task, executes dependency stages, and finalizes the
//! parent-facing [`SwarmResult`](crate::swarm::SwarmResult). `Run` carries the
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
mod stage;
mod stage_stats;
mod stages;
mod state;
mod store;

use super::SwarmExecutor;
use crate::swarm::{DecompositionStrategy, SwarmResult};
use anyhow::Result;

pub(super) async fn execute(
    executor: &SwarmExecutor,
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
