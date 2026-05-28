//! Common worker initialization used by both entrypoints.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::{bus::AgentBus, cli::A2aArgs};

use super::{
    HeartbeatState, WorkerContext, WorkerTaskRuntime, build_worker_http_client,
    export_worker_runtime_env, maybe_start_worker_a2a_peer, normalize_max_concurrent_tasks,
    parse_auto_approve, resolve_worker_id, task_timeline, worker_capabilities,
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
    let shared_codebases = Arc::new(Mutex::new(codebases(&args)));
    let client = build_worker_http_client()?;
    let processing = super::worker_init_runtime::processing_set();
    let cognition_heartbeat = super::CognitionHeartbeatConfig::from_env();
    let heartbeat_state = HeartbeatState::new(worker_id.clone(), name.clone());
    let bus = AgentBus::new().into_arc();
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
    bus.handle(&worker_id).announce_ready(worker_capabilities());
    let _a2a_peer = maybe_start_worker_a2a_peer(&args, &name, bus.clone()).await;
    let task_progress = Arc::new(Mutex::new(task_timeline::TaskProgressState::new()));
    let task_runtime = runtime(
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
    })
}

fn runtime(
    args: &A2aArgs,
    client: &reqwest::Client,
    server: &str,
    worker_id: &str,
    name: &str,
    processing: &super::worker_init_runtime::ProcessingSet,
    bus: &Arc<AgentBus>,
    task_progress: &Arc<Mutex<task_timeline::TaskProgressState>>,
) -> WorkerTaskRuntime {
    WorkerTaskRuntime {
        client: client.clone(),
        server: server.to_string(),
        token: args.token.clone(),
        worker_id: worker_id.to_string(),
        agent_name: name.to_string(),
        processing: processing.clone(),
        max_concurrent_tasks: normalize_max_concurrent_tasks(args.max_concurrent_tasks),
        auto_approve: parse_auto_approve(&args.auto_approve),
        bus: bus.clone(),
        task_progress: task_progress.clone(),
        workspace_ids: Vec::new(),
    }
}

fn codebases(args: &A2aArgs) -> Vec<String> {
    args.workspaces
        .as_deref()
        .map(|items| {
            items
                .split(',')
                .map(|item| item.trim().to_string())
                .collect()
        })
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()])
}
