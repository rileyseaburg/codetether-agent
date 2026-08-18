//! Governance sweep and proposal execution for one tick.

use chrono::{DateTime, Utc};

use super::loop_ctx::LoopCtx;
use super::tick_process::TickOutput;
use super::{proposal_execute, vote_sweep};

/// Resolve pending proposals, then execute any that became verified.
pub(super) async fn govern(ctx: &LoopCtx, now: DateTime<Utc>, out: &mut TickOutput) {
    vote_sweep::resolve_pending(
        &ctx.proposals,
        &ctx.personas,
        &ctx.governance,
        &ctx.attention_queue,
        now,
    )
    .await;
    out.events.extend(
        proposal_execute::execute_verified(proposal_execute::ExecuteCtx {
            proposals: &ctx.proposals,
            receipts: &ctx.receipts,
            pending_approvals: &ctx.pending_approvals,
        })
        .await,
    );
}

/// Reap idle personas and publish their reap events.
pub(super) async fn reap_idle(ctx: &LoopCtx, now: DateTime<Utc>) {
    let events = {
        let mut personas = ctx.personas.write().await;
        super::tick_idle_reap::reap_idle(&mut personas, now)
    };
    for event in events {
        super::buffers::push_event_internal(&ctx.events, ctx.max_events, &ctx.event_tx, event)
            .await;
    }
}
