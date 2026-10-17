use super::*;
pub(super) async fn execute(runtime: &mut impl StageRuntime, stage: usize) -> Result<()> {
    let started = Instant::now();
    let (runnable, mut results) = {
        let input = runtime.stage_input(stage);
        if input.tasks.is_empty() {
            return Ok(());
        }
        tracing::info!(provider = %input.provider, stage, count = input.tasks.len(),
            "Executing swarm stage");
        let (runnable, results) = crate::run_policy::partition(&input.tasks, input.failed);
        runtime.report_blocked(&input.tasks, &results);
        (runnable, results)
    };
    let mut executed = runtime.execute_tasks(runnable).await?;
    results.append(&mut executed);
    runtime.record_results(stage, started, results).await;
    Ok(())
}
