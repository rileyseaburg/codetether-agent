use crate::cognition::{ThinkerClient, ThoughtWorkItem, loop_ctx::LoopCtx};
use crate::tool::ToolRegistry;
use chrono::{DateTime, Utc};
use codetether_cognition_core::{
    runtime::process::{ClientServices, process_work},
    thought::TickOutput,
};
pub(in crate::cognition) struct Native;
impl codetether_cognition_core::runtime::tick::Host<ThinkerClient, ToolRegistry> for Native {
    async fn process_work(
        &self,
        ctx: &LoopCtx,
        work: &ThoughtWorkItem,
        active: usize,
        now: DateTime<Utc>,
        out: &mut TickOutput,
    ) {
        process_work(ctx, work, active, now, out, &ClientServices).await;
    }
    async fn persist(&self, ctx: &LoopCtx) {
        codetether_cognition_storage::persist_tick(ctx).await;
    }
}
