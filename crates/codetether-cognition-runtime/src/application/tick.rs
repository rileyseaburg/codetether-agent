//! Thought processing and best-effort persistence for shared thinker clients.
use chrono::{DateTime, Utc};
use codetether_cognition_core::{
    ThoughtWorkItem,
    execution::dispatch::Registry,
    runtime::{
        LoopContext,
        process::{ClientServices, process_work},
        tick::Host,
    },
    thinker::client::{Client, Host as ClientHost},
    thought::TickOutput,
};
/// Tick services that process thoughts and persist completed compression ticks.
/// # Examples
/// ```no_run
/// # async fn tick<H: codetether_cognition_core::thinker::client::Host, R: codetether_cognition_core::execution::dispatch::Registry>(context: &codetether_cognition_core::runtime::LoopContext<codetether_cognition_core::thinker::client::Client<H>, R>) {
/// codetether_cognition_runtime::run_tick(context, &codetether_cognition_runtime::application::TickServices).await;
/// # }
/// ```
pub struct TickServices;
impl<H: ClientHost, R: Registry> Host<Client<H>, R> for TickServices {
    async fn process_work(
        &self,
        ctx: &LoopContext<Client<H>, R>,
        work: &ThoughtWorkItem,
        active: usize,
        now: DateTime<Utc>,
        out: &mut TickOutput,
    ) {
        process_work(ctx, work, active, now, out, &ClientServices).await;
    }
    async fn persist(&self, ctx: &LoopContext<Client<H>, R>) {
        codetether_cognition_storage::persist_tick(ctx).await;
    }
}
