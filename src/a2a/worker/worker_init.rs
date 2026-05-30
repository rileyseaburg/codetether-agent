//! Common worker initialization used by both entrypoints.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::{bus::AgentBus, cli::A2aArgs};

use super::{
    HeartbeatState, WorkerContext, build_worker_http_client, export_worker_runtime_env,
    maybe_start_worker_a2a_peer, resolve_worker_id, task_timeline, worker_capabilities,
    worker_init_helpers,
};

pub(super) async fn init_worker(args: A2aArgs) -> Result<WorkerContext> {
    let server = args.server.trim_end_matches('/').to_string();
    let name = args
        .name
        .as_deref()
        .map(ToString::to_string)
        .unwrap_or_else(|| format!("codetether-{}", std::process::id()));
    let worker_id = resolve_worker_id();
    export_worker_runtime_env(&server, &args.token, &worker_id);
    let shared_codebases = Arc::new(Mutex::new(worker_init_helpers::parse_codebases(&args)));
    let client = build_worker_http_client()?;
    let processing = super::worker_init_runtime::processing_set();
    let cognition_heartbeat = super::CognitionHeartbeatConfig::from_env();
    let heartbeat_state = HeartbeatState::new(worker_id.clone(), name.clone());
    let bus = AgentBus::new().into_arc();
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
    bus.handle(&worker_id).announce_ready(worker_capabilities());
    let a2a_peer = maybe_start_worker_a2a_peer(&args, &name, bus.clone()).await;
    let task_progress = Arc::new(Mutex::new(task_timeline::TaskProgressState::new()));
    let task_runtime = worker_init_helpers::build_task_runtime(
        &args,
        &client,
        &server,
        &worker_id,
        &name,
        &processing,
        &bus,
        &task_progress,
    );
    tracing::info!(
        "Starting A2A worker: {} ({}) server={} workspaces={:?} max_concurrent={}",
        name,
        worker_id,
        server,
        shared_codebases.lock().await.clone(),
        task_runtime.max_concurrent_tasks
    );
    Ok(WorkerContext {
        args,
        server,
        name,
        worker_id,
        shared_codebases,
        processing,
        cognition_heartbeat,
        heartbeat_state,
        bus,
        task_progress,
        task_runtime,
        a2a_peer,
    })
}
