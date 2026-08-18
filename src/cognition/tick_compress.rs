//! Compress-phase snapshotting, belief decay, and workspace refresh.

use chrono::{DateTime, Utc};

use super::loop_ctx::LoopCtx;
use super::tick_process::TickOutput;
use super::{ThoughtResult, ThoughtWorkItem, belief_decay, tick_snapshot, workspace_refresh};

/// Snapshot memory, decay stale beliefs, and refresh the shared workspace.
pub(super) async fn compress(
    ctx: &LoopCtx,
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
