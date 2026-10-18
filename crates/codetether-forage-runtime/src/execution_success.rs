//! Persist successful execution evidence before publishing completion.
use super::*;
pub(super) async fn record(
    repo: &OkrRepository,
    item: &ForageOpportunity,
    args: &ForageArgs,
    execution_outcome: ExecutionOutcome,
    cycle: usize,
    events: &events::RunEvents<'_>,
    exec_task_id: &str,
) {
    if let Err(err) =
        record_execution_success_to_okr(repo, item, args, &execution_outcome, cycle).await
    {
        tracing::warn!(target: "codetether_agent::forage",
            okr_id = %item.okr_id,
            key_result_id = %item.key_result_id,
            error = %err,
            "Failed to persist forage execution progress to OKR"
        );
    }
    events.success(item, exec_task_id, execution_outcome).await;
}
