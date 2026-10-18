//! Lifecycle driver for a local-thread stage.

use crate::Host;
use anyhow::Result;
use codetether_swarm_core::{SubTask, SubTaskResult};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

/// Execute one dependency stage using isolated local jobs.
/// # Arguments
/// Receives native services, provider registry/default selection, tasks, prior results and swarm ID.
/// # Returns
/// Cached and executed results after worktree integration.
/// # Errors
/// Returns missing-provider errors during stage setup.
/// # Examples
/// ```no_run
/// async fn stage<H: codetether_swarm_local_stage::Host>(host: &H, providers: &H::Providers) -> anyhow::Result<()> {
///     let prior = std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
///     codetether_swarm_local_stage::execute(host, providers, "provider", "model", Vec::new(), prior, "swarm").await?;
///     Ok(())
/// }
/// ```
pub async fn execute<H: Host>(
    executor: &H,
    providers: &H::Providers,
    provider_name: &str,
    model: &str,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> Result<Vec<SubTaskResult>> {
    let mut state = super::create::state(
        executor,
        providers,
        provider_name,
        model,
        &tasks,
        completed,
        swarm,
    )?;
    let pending = super::cache_filter::pending(&mut state, tasks).await;
    let runnable = super::worktrees::prepare(&mut state, pending).await;
    super::launch::all(&mut state, runnable).await;
    super::collect::all(&mut state).await;
    Ok(super::integrate::all(&mut state).await)
}
