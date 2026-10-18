use super::Host;
use crate::initialization::WorkerContext;
use crate::{HeartbeatState, export_worker_runtime_env, resolve_worker_id, task_timeline};
use std::sync::Arc;
use tokio::sync::Mutex;
/// Construct worker state and start native startup services in their original order.
/// # Arguments
/// * `args` — Original native worker arguments, retained in the context.
/// # Returns
/// Shared worker identity, task runtime, bus, heartbeat and optional peer state.
/// # Errors
/// Returns HTTP-client initialization errors before bus services start.
/// # Examples
/// ```ignore
/// let context = init_worker::<Native>(args).await?;
/// ```
pub async fn init_worker<H: Host>(
    args: H::Args,
) -> anyhow::Result<WorkerContext<H::Args, H::Client, H::Bus, H::Peer>> {
    H::apply_access(&args);
    let options = H::options(&args);
    let server = options.server.trim_end_matches('/').to_string();
    let name = super::super::resolve_name(options.name);
    let worker_id = resolve_worker_id();
    export_worker_runtime_env(&server, options.token, &worker_id);
    let shared_codebases = Arc::new(Mutex::new(super::super::parse_codebases(
        options.workspaces,
    )));
    let client = H::client()?;
    let processing = super::super::processing_set();
    let cognition_heartbeat = crate::CognitionHeartbeatConfig::from_env();
    let heartbeat_state = HeartbeatState::new(worker_id.clone(), name.clone());
    let bus = H::bus();
    let _announcement = H::announce(&bus, &worker_id);
    let a2a_peer = H::peer(&args, &name, bus.clone()).await;
    let task_progress = Arc::new(Mutex::new(task_timeline::TaskProgressState::new()));
    let task_runtime = super::super::build_task_runtime(
        &client,
        super::super::RuntimeConfig {
            server: &server,
            token: options.token.as_deref(),
            worker_id: &worker_id,
            name: &name,
            max_concurrent_tasks: options.max_concurrent_tasks,
            auto_approve: options.auto_approve,
        },
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
