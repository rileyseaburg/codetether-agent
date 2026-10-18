//! Execute one opportunity with the application swarm service.
use crate::*;

use std::time::Duration;

pub(crate) async fn execute_opportunity_with_swarm<H: Host>(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    let swarm_config = build_swarm_config(args);
    let executor = H::swarm(swarm_config);
    let strategy = parse_swarm_strategy(&args.swarm_strategy);

    match tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        executor.execute(&item.prompt, strategy),
    )
    .await
    {
        Ok(Ok(result)) => {
            if result.success {
                Ok(format!(
                    "swarm execution completed (subagents_spawned={}, completed={}, failed={}, retries={})",
                    result.stats.subagents_spawned,
                    result.stats.subagents_completed,
                    result.stats.subagents_failed,
                    result
                        .subtask_results
                        .iter()
                        .map(|r| r.retry_count)
                        .sum::<u32>() as usize
                ))
            } else {
                let error = result.error.unwrap_or_else(|| {
                    format!(
                        "swarm reported failure (failed_subtasks={}, total_subtasks={})",
                        result.stats.subagents_failed, result.stats.subagents_spawned
                    )
                });
                anyhow::bail!(error);
            }
        }
        Ok(Err(err)) => Err(err),
        Err(_) => anyhow::bail!("swarm execution timed out after {timeout_secs}s"),
    }
}
