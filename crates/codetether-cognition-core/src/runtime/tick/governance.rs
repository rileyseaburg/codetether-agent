use crate::{
    proposal::{dispatch as proposal_execute, voting as vote_sweep},
    runtime::LoopContext,
    thought::TickOutput,
};
use chrono::{DateTime, Utc};
/// Resolve pending proposals, then execute any that became verified.
pub(super) async fn govern<T, R>(
    ctx: &LoopContext<T, R>,
    now: DateTime<Utc>,
    out: &mut TickOutput,
) {
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
pub(super) async fn reap_idle<T, R>(ctx: &LoopContext<T, R>, now: DateTime<Utc>) {
    let events = {
        let mut personas = ctx.personas.write().await;
        crate::persona::scheduling::reap_idle(&mut personas, now)
    };
    for event in events {
        crate::thought::buffers::push_event_internal(
            &ctx.events,
            ctx.max_events,
            &ctx.event_tx,
            event,
        )
        .await;
    }
}
