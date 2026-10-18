//! Select bridge commands, observability events and process shutdown.
use crate::{WorkerBridgeCmd, identity::Identity};
use codetether_a2a_worker_core::HeartbeatState;
use codetether_bus_core::AgentBus;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::mpsc;
pub(crate) async fn run(
    id: &Identity,
    client: &Client,
    heartbeat_state: &HeartbeatState,
    bus: Arc<AgentBus>,
    mut cmd_rx: mpsc::Receiver<WorkerBridgeCmd>,
) {
    let mut bus_handle = bus.handle(&id.worker_id);
    loop {
        tokio::select! {
         Some(cmd)=cmd_rx.recv()=>{if crate::commands::apply(cmd,heartbeat_state).await {break;}},
         Some(envelope)=bus_handle.recv()=>{if let Err(e)=codetether_worker_events::forward_bus_event(client,&id.server,&id.token,&id.worker_id,&envelope).await {tracing::debug!("Failed to forward bus event: {}",e);}},
         _=tokio::signal::ctrl_c()=>{tracing::info!("Worker bridge received shutdown signal");break;}
        }
    }
}
