//! Start the shared heartbeat scheduler with an empty task timeline.
use crate::context::Context;
use codetether_a2a_worker_core::task_timeline::TaskProgressState;
use std::sync::Arc;
use tokio::sync::Mutex;
pub(crate) fn start(ctx: &Context, token_opt: Option<String>) -> tokio::task::JoinHandle<()> {
    codetether_a2a_http::worker::start_heartbeat(
        ctx.client.clone(),
        ctx.identity.server.clone(),
        token_opt,
        ctx.heartbeat_state.clone(),
        ctx.processing.clone(),
        ctx.cognition_config.clone(),
        Arc::new(Mutex::new(TaskProgressState::new())),
    )
}
