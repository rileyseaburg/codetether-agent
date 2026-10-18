//! Native state adapters for shared worker reconnection loops.
mod mode;
mod server;
mod worker;
pub(super) struct Native<'a, S> {
    pub context: &'a super::super::WorkerContext,
    pub state: S,
}
