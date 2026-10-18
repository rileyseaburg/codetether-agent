//! Execute selected opportunities and retain counters and fail-fast behavior.
use super::*;
pub(super) async fn execute_selected<H: Host>(
    args: &ForageArgs,
    repo: &OkrRepository,
    events: &events::RunEvents<'_>,
    selected: &[ForageOpportunity],
    summary: &mut ForageRunSummary,
    cycle: usize,
) -> Result<()> {
    if args.execute && !selected.is_empty() {
        for item in selected {
            let exec_task_id = events.executing(item);
            summary.total_executed = summary.total_executed.saturating_add(1);
            match codetether_forage_execution::execute_opportunity::<H>(item, args).await {
                Ok(execution_outcome) => {
                    execution_success::record(
                        repo,
                        item,
                        args,
                        execution_outcome,
                        cycle,
                        events,
                        &exec_task_id,
                    )
                    .await;
                }
                Err(err) => {
                    summary.total_execution_failures =
                        summary.total_execution_failures.saturating_add(1);
                    events.failure(item, &exec_task_id, &err).await;
                    if args.fail_fast {
                        return Err(err);
                    }
                }
            }
        }
    }

    Ok(())
}
