//! Worker-server bridge setup helpers.

use std::sync::Arc;

use crate::worker_server::WorkerServerState;

use super::WorkerContext;

pub(super) async fn attach_server_state(context: &WorkerContext, server_state: &WorkerServerState) {
    server_state.set_worker_id(context.worker_id.clone()).await;
    server_state
        .set_heartbeat_state(Arc::new(context.heartbeat_state.clone()))
        .await;
    server_state.set_bus(context.bus.clone()).await;
}
