//! One cognition tick from work selection through publication and persistence.
//!
//! [run_tick] keeps work, governance, publication and reap order in one place.
//! Implement [Host] to supply per-thought application execution and persistence.
mod governance;
mod publication;
mod run;
use super::LoopContext;
use crate::{ThoughtWorkItem, thought::TickOutput};
use chrono::{DateTime, Utc};
pub use run::run_tick;
use std::future::Future;
/// Native operations retained outside shared tick sequencing.
/// # Implementors
/// Applications bind thought generation/tool execution and persistence policy.
pub trait Host<T, R>: Sync {
    /// Process one selected work item and append its output to the tick buffer.
    fn process_work(
        &self,
        ctx: &LoopContext<T, R>,
        work: &ThoughtWorkItem,
        active: usize,
        now: DateTime<Utc>,
        out: &mut TickOutput,
    ) -> impl Future<Output = ()> + Send;
    /// Persist the state after a tick that includes a Compress-phase item.
    fn persist(&self, ctx: &LoopContext<T, R>) -> impl Future<Output = ()> + Send;
}
