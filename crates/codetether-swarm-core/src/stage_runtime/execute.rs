use super::*;
/// Traverse dependency stages with a readiness check before each stage.
///
/// # Arguments
/// * `runtime` - Host state implementing backend and result-recording operations.
///
/// # Returns
/// Success after the last stage or after a graceful readiness stop.
///
/// # Errors
/// Returns the first backend execution error without recording that stage.
///
/// # Examples
/// ```no_run
/// # async fn example(runtime: &mut impl codetether_swarm_core::stage_runtime::StageRuntime) -> anyhow::Result<()> {
/// codetether_swarm_core::stage_runtime::execute_stages(runtime).await?;
/// # Ok(())
/// # }
/// ```
pub async fn execute_stages(runtime: &mut impl StageRuntime) -> Result<()> {
    let maximum = runtime
        .subtasks()
        .iter()
        .map(|task| task.stage)
        .max()
        .unwrap_or(0);
    for stage in 0..=maximum {
        if !runtime.may_continue().await {
            break;
        }
        super::stage::execute(runtime, stage).await?;
    }
    Ok(())
}
