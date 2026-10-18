use crate::{ThoughtResult, ThoughtWorkItem, runtime::LoopContext, thought::TickOutput};
use crate::{
    beliefs::update as belief_decay, thought::build as tick_snapshot,
    workspace::refresh as workspace_refresh,
};
use chrono::{DateTime, Utc};
/// Snapshot memory, decay stale beliefs, and refresh the shared workspace.
pub(super) async fn compress<T, R>(
    ctx: &LoopContext<T, R>,
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    hot_event_count: usize,
    now: DateTime<Utc>,
    out: &mut TickOutput,
) {
    out.snapshots.push(tick_snapshot::build_snapshot(
        work,
        thought,
        hot_event_count,
    ));
    {
        let mut store = ctx.beliefs.write().await;
        let mut queue = ctx.attention_queue.write().await;
        belief_decay::decay_stale_beliefs(&mut store, &mut queue, now);
    }
    workspace_refresh::refresh_workspace(&ctx.beliefs, &ctx.attention_queue, &ctx.workspace, now)
        .await;
    out.events.push(workspace_refresh::updated_event(work));
}
