//! Native state adapters for shared worker reconnection loops.
mod mode;
mod server;
mod worker;
pub(super) struct Native<'a, D, S> {
    pub context: &'a crate::WorkerContext,
    pub state: S,
    pub dispatcher: std::marker::PhantomData<fn() -> D>,
}

use crate::{WorkerContext, pending::Dispatcher};
use codetether_worker_http::WorkerServerState;
use std::marker::PhantomData;
/// Run standalone worker reconnection with the supplied task dispatcher.
/// Errors are returned by the shared reconnect loop.
pub async fn run_worker_loop<D: Dispatcher>(context: WorkerContext) -> anyhow::Result<()> {
    codetether_a2a_stream::worker_reconnect::run_worker_loop(&Native::<D, _> {
        context: &context,
        state: (),
        dispatcher: PhantomData,
    })
    .await
}
/// Run reconnection integrated with worker HTTP notifications and status.
/// Errors are returned by the shared reconnect loop.
pub async fn run_worker_server_loop<D: Dispatcher>(
    context: WorkerContext,
    server_state: WorkerServerState,
) -> anyhow::Result<()> {
    codetether_a2a_stream::worker_reconnect::run_worker_server_loop(&Native::<D, _> {
        context: &context,
        state: &server_state,
        dispatcher: PhantomData,
    })
    .await
}
