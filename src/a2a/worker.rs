//! A2A Worker - connects to an A2A server to process tasks

mod clone_location;
mod clone_target;
mod model_defaults;
mod model_preferences;
pub(crate) mod task_timeline;
pub(super) mod workspace_resolve;

use crate::a2a::claim::TaskClaimResponse;
use crate::a2a::git_credentials::{
    configure_repo_git_auth, configure_repo_git_github_app, write_git_credential_helper_script,
};
use crate::a2a::worker_workspace_record::{RegisteredWorkspaceRecord, fetch_workspace_record};
use crate::bus::AgentBus;
use crate::cli::{A2aArgs, ForageArgs};
use crate::provenance::{ClaimProvenance, install_commit_msg_hook};
use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};
use crate::tui::swarm_view::SwarmEvent;
use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;
use tokio::time::Instant;

use self::model_defaults::{default_model_for_provider, prefers_temperature_one};
use self::model_preferences::choose_provider_for_tier;
use crate::a2a::task_scope::check_task_scope;
use crate::a2a::worker_tool_registry::{create_filtered_registry, is_tool_allowed};
use crate::a2a::worker_workspace_context::resolve_task_workspace_dir;
use clone_location::{git_clone_base_dir, resolve_workspace_clone_path};
use clone_target::prepare_clone_target;

/// Worker status for heartbeat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStatus {
    Idle,
    Processing,
}

impl WorkerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkerStatus::Idle => "idle",
            WorkerStatus::Processing => "processing",
        }
    }
}

/// Heartbeat state shared between the heartbeat task and the main worker
#[derive(Clone)]
pub struct HeartbeatState {
    worker_id: String,
    pub agent_name: String,
    pub status: Arc<Mutex<WorkerStatus>>,
    pub active_task_count: Arc<Mutex<usize>>,
    pub sub_agents: Arc<Mutex<HashSet<String>>>,
}

impl HeartbeatState {
    pub fn new(worker_id: String, agent_name: String) -> Self {
        Self {
            worker_id,
            agent_name,
            status: Arc::new(Mutex::new(WorkerStatus::Idle)),
            active_task_count: Arc::new(Mutex::new(0)),
            sub_agents: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub async fn set_status(&self, status: WorkerStatus) {
        *self.status.lock().await = status;
    }

    pub async fn set_task_count(&self, count: usize) {
        *self.active_task_count.lock().await = count;
    }

    pub async fn register_sub_agent(&self, agent_name: String) {
        self.sub_agents.lock().await.insert(agent_name);
    }

    pub async fn deregister_sub_agent(&self, agent_name: &str) {
        self.sub_agents.lock().await.remove(agent_name);
    }

    pub async fn sub_agents_snapshot(&self) -> Vec<String> {
        let mut names = self
            .sub_agents
            .lock()
            .await
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        names.sort();
        names
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CognitionHeartbeatConfig {
    pub enabled: bool,
    pub source_base_url: String,
    pub token: Option<String>,
    pub provider_name: String,
    pub interval_secs: u64,
    pub include_thought_summary: bool,
    pub summary_max_chars: usize,
    pub request_timeout_ms: u64,
}

impl CognitionHeartbeatConfig {
    pub fn from_env() -> Self {
        let source_base_url = std::env::var("CODETETHER_WORKER_COGNITION_SOURCE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4096".to_string())
            .trim_end_matches('/')
            .to_string();

        Self {
            enabled: env_bool("CODETETHER_WORKER_COGNITION_SHARE_ENABLED", true),
            source_base_url,
            include_thought_summary: env_bool("CODETETHER_WORKER_COGNITION_INCLUDE_THOUGHTS", true),
            summary_max_chars: env_usize("CODETETHER_WORKER_COGNITION_THOUGHT_MAX_CHARS", 480),
            request_timeout_ms: env_u64("CODETETHER_WORKER_COGNITION_TIMEOUT_MS", 2_500).max(250),
            interval_secs: env_u64("CODETETHER_WORKER_COGNITION_INTERVAL_SECS", 30).max(5),
            provider_name: std::env::var("CODETETHER_WORKER_COGNITION_PROVIDER")
                .unwrap_or_else(|_| "cognition".to_string()),
            token: std::env::var("CODETETHER_WORKER_COGNITION_TOKEN").ok(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CognitionStatusSnapshot {
    running: bool,
    #[serde(default)]
    last_tick_at: Option<String>,
    #[serde(default)]
    active_persona_count: usize,
    #[serde(default)]
    events_buffered: usize,
    #[serde(default)]
    snapshots_buffered: usize,
    #[serde(default)]
    loop_interval_ms: u64,
}

#[derive(Debug, Deserialize)]
struct CognitionLatestSnapshot {
    generated_at: String,
    summary: String,
    #[serde(default)]
    metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskReservation {
    Reserved,
    AlreadyProcessing,
    AtCapacity,
}

#[derive(Clone)]
struct WorkerTaskRuntime {
    client: Client,
    server: String,
    token: Option<String>,
    worker_id: String,
    agent_name: String,
    processing: Arc<Mutex<HashSet<String>>>,
    max_concurrent_tasks: usize,
    auto_approve: AutoApprove,
    bus: Arc<AgentBus>,
    /// Shared progress state for the currently active task (read by heartbeat).
    task_progress: Arc<Mutex<task_timeline::TaskProgressState>>,
    /// Server-side workspace IDs this worker is scoped to (empty = accept all).
    workspace_ids: Vec<String>,
}

// Run the A2A worker
pub async fn run(args: A2aArgs) -> Result<()> {
    let server = args.server.trim_end_matches('/');
    let name = args
        .name
        .unwrap_or_else(|| format!("codetether-{}", std::process::id()));
    let worker_id = resolve_worker_id();
    export_worker_runtime_env(server, &args.token, &worker_id);
    let max_concurrent_tasks = normalize_max_concurrent_tasks(args.max_concurrent_tasks);

    let codebases: Vec<String> = args
        .workspaces
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()]);

    tracing::info!("Starting A2A worker: {} ({})", name, worker_id);
    tracing::info!("Server: {}", server);
    tracing::info!("Workspaces: {:?}", codebases);
    tracing::info!(max_concurrent_tasks, "Worker task concurrency configured");

    // Wrap in shared mutex so background workspace-sync can add new local paths
    let shared_codebases: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(codebases));

    let client = Client::new();
    let processing = Arc::new(Mutex::new(HashSet::<String>::new()));
    let cognition_heartbeat = CognitionHeartbeatConfig::from_env();
    if cognition_heartbeat.enabled {
        tracing::info!(
            source = %cognition_heartbeat.source_base_url,
            include_thoughts = cognition_heartbeat.include_thought_summary,
            max_chars = cognition_heartbeat.summary_max_chars,
            timeout_ms = cognition_heartbeat.request_timeout_ms,
            "Cognition heartbeat sharing enabled (set CODETETHER_WORKER_COGNITION_SHARE_ENABLED=false to disable)"
        );
    } else {
        tracing::warn!(
            "Cognition heartbeat sharing disabled; worker thought state will not be shared upstream"
        );
    }

    let auto_approve = match args.auto_approve.as_str() {
        "all" => AutoApprove::All,
        "safe" => AutoApprove::Safe,
        _ => AutoApprove::None,
    };

    // Create heartbeat state
    let heartbeat_state = HeartbeatState::new(worker_id.clone(), name.clone());

    // Create agent bus for in-process sub-agent communication
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    {
        let handle = bus.handle(&worker_id);
        handle.announce_ready(worker_capabilities());
    }

    let task_progress: Arc<Mutex<task_timeline::TaskProgressState>> =
        Arc::new(Mutex::new(task_timeline::TaskProgressState::new()));

    let task_runtime = WorkerTaskRuntime {
        client: client.clone(),
        server: server.to_string(),
        token: args.token.clone(),
        worker_id: worker_id.clone(),
        agent_name: name.clone(),
        processing: processing.clone(),
        max_concurrent_tasks,
        auto_approve,
        bus: bus.clone(),
        task_progress: task_progress.clone(),
        workspace_ids: Vec::new(),
    };

    // Register worker
    {
        let codebases = shared_codebases.lock().await.clone();
        register_worker(
            &client,
            server,
            &args.token,
            &worker_id,
            &name,
            &codebases,
            args.public_url.as_deref(),
        )
        .await?;
    }

    if let Err(e) =
        sync_workspaces_from_server(&client, server, &args.token, &shared_codebases).await
    {
        tracing::warn!(error = %e, "Initial workspace sync failed");
    }

    // Fetch pending tasks
    fetch_pending_tasks(&task_runtime).await?;

    // Start background task that polls server for new locally-present workspaces
    let _workspace_sync_handle = start_workspace_sync(
        client.clone(),
        server.to_string(),
        args.token.clone(),
        shared_codebases.clone(),
    );

    // Connect to SSE stream
    loop {
        // Refresh codebases snapshot (background sync may have added new local paths)
        let codebases = shared_codebases.lock().await.clone();

        // Re-register worker on each reconnection to report updated models/capabilities
        if let Err(e) = register_worker(
            &client,
            server,
            &args.token,
            &worker_id,
            &name,
            &codebases,
            args.public_url.as_deref(),
        )
        .await
        {
            tracing::warn!("Failed to re-register worker on reconnection: {}", e);
        }
        if let Err(e) = fetch_pending_tasks(&task_runtime).await {
            tracing::warn!("Reconnect task fetch failed: {}", e);
        }

        // Start heartbeat task for this connection
        let heartbeat_handle = start_heartbeat(
            client.clone(),
            server.to_string(),
            args.token.clone(),
            heartbeat_state.clone(),
            processing.clone(),
            cognition_heartbeat.clone(),
            task_progress.clone(),
        );

        match connect_stream(&task_runtime, &name, &codebases, None).await {
            Ok(()) => {
                tracing::warn!("Stream ended, reconnecting...");
            }
            Err(e) => {
                tracing::error!("Stream error: {}, reconnecting...", e);
            }
        }

        // Cancel heartbeat on disconnection
        heartbeat_handle.abort();
        tracing::debug!("Heartbeat cancelled for reconnection");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

/// Run the A2A worker with shared state for HTTP server integration
/// This variant accepts a WorkerServerState to communicate with the HTTP server
pub async fn run_with_state(
    args: A2aArgs,
    server_state: crate::worker_server::WorkerServerState,
) -> Result<()> {
    let server = args.server.trim_end_matches('/');
    let name = args
        .name
        .unwrap_or_else(|| format!("codetether-{}", std::process::id()));
    let worker_id = resolve_worker_id();
    export_worker_runtime_env(server, &args.token, &worker_id);
    let max_concurrent_tasks = normalize_max_concurrent_tasks(args.max_concurrent_tasks);

    // Share worker_id with HTTP server
    server_state.set_worker_id(worker_id.clone()).await;

    let codebases: Vec<String> = args
        .workspaces
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()]);

    tracing::info!("Starting A2A worker: {} ({})", name, worker_id);
    tracing::info!("Server: {}", server);
    tracing::info!("Workspaces: {:?}", codebases);
    tracing::info!(max_concurrent_tasks, "Worker task concurrency configured");

    // Wrap in shared mutex so background workspace-sync can add new local paths
    let shared_codebases: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(codebases));

    let client = Client::new();
    let processing = Arc::new(Mutex::new(HashSet::<String>::new()));
    let cognition_heartbeat = CognitionHeartbeatConfig::from_env();
    if cognition_heartbeat.enabled {
        tracing::info!(
            source = %cognition_heartbeat.source_base_url,
            include_thoughts = cognition_heartbeat.include_thought_summary,
            max_chars = cognition_heartbeat.summary_max_chars,
            timeout_ms = cognition_heartbeat.request_timeout_ms,
            "Cognition heartbeat sharing enabled (set CODETETHER_WORKER_COGNITION_SHARE_ENABLED=false to disable)"
        );
    } else {
        tracing::warn!(
            "Cognition heartbeat sharing disabled; worker thought state will not be shared upstream"
        );
    }

    let auto_approve = match args.auto_approve.as_str() {
        "all" => AutoApprove::All,
        "safe" => AutoApprove::Safe,
        _ => AutoApprove::None,
    };

    // Create heartbeat state
    let heartbeat_state = HeartbeatState::new(worker_id.clone(), name.clone());

    // Share heartbeat state with HTTP server
    server_state
        .set_heartbeat_state(Arc::new(heartbeat_state.clone()))
        .await;

    // Create agent bus for in-process sub-agent communication
    let bus = AgentBus::new().into_arc();
    server_state.set_bus(bus.clone()).await;

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    {
        let handle = bus.handle(&worker_id);
        handle.announce_ready(worker_capabilities());
    }

    let task_progress_ws: Arc<Mutex<task_timeline::TaskProgressState>> =
        Arc::new(Mutex::new(task_timeline::TaskProgressState::new()));

    let task_runtime = WorkerTaskRuntime {
        client: client.clone(),
        server: server.to_string(),
        token: args.token.clone(),
        worker_id: worker_id.clone(),
        agent_name: name.clone(),
        processing: processing.clone(),
        max_concurrent_tasks,
        auto_approve,
        bus: bus.clone(),
        task_progress: task_progress_ws.clone(),
        workspace_ids: Vec::new(),
    };

    // Register worker
    {
        let codebases = shared_codebases.lock().await.clone();
        register_worker(
            &client,
            server,
            &args.token,
            &worker_id,
            &name,
            &codebases,
            args.public_url.as_deref(),
        )
        .await?;
    }

    if let Err(e) =
        sync_workspaces_from_server(&client, server, &args.token, &shared_codebases).await
    {
        tracing::warn!(error = %e, "Initial workspace sync failed");
    }

    // Mark as connected
    server_state.set_connected(true).await;

    // Fetch pending tasks before entering reconnection loop
    fetch_pending_tasks(&task_runtime).await?;

    // Start background task that polls server for new locally-present workspaces
    let _workspace_sync_handle = start_workspace_sync(
        client.clone(),
        server.to_string(),
        args.token.clone(),
        shared_codebases.clone(),
    );

    // Connect to SSE stream
    loop {
        // Refresh codebases snapshot (background sync may have added new local paths)
        let codebases = shared_codebases.lock().await.clone();

        // Create task notification channel for CloudEvent-triggered task execution
        // Recreate on each reconnection since the receiver is moved into connect_stream
        let (task_notify_tx, task_notify_rx) = mpsc::channel::<String>(32);
        server_state
            .set_task_notification_channel(task_notify_tx)
            .await;

        // Mark as connected on each reconnection
        server_state.set_connected(true).await;

        // Re-register worker on each reconnection to report updated models/capabilities
        if let Err(e) = register_worker(
            &client,
            server,
            &args.token,
            &worker_id,
            &name,
            &codebases,
            args.public_url.as_deref(),
        )
        .await
        {
            tracing::warn!("Failed to re-register worker on reconnection: {}", e);
        }
        if let Err(e) = fetch_pending_tasks(&task_runtime).await {
            tracing::warn!("Reconnect task fetch failed: {}", e);
        }

        // Start heartbeat task for this connection
        let heartbeat_handle = start_heartbeat(
            client.clone(),
            server.to_string(),
            args.token.clone(),
            heartbeat_state.clone(),
            processing.clone(),
            cognition_heartbeat.clone(),
            task_progress_ws.clone(),
        );

        match connect_stream(&task_runtime, &name, &codebases, Some(task_notify_rx)).await {
            Ok(()) => {
                tracing::warn!("Stream ended, reconnecting...");
            }
            Err(e) => {
                tracing::error!("Stream error: {}, reconnecting...", e);
            }
        }

        // Mark as disconnected
        server_state.set_connected(false).await;

        // Cancel heartbeat on disconnection
        heartbeat_handle.abort();
        tracing::debug!("Heartbeat cancelled for reconnection");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

pub fn generate_worker_id() -> String {
    format!(
        "wrk_{}_{:x}",
        chrono::Utc::now().timestamp(),
        rand::random::<u64>()
    )
}

fn resolve_worker_id() -> String {
    for key in ["CODETETHER_WORKER_ID", "A2A_WORKER_ID"] {
        if let Ok(value) = std::env::var(key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    generate_worker_id()
}

fn normalize_max_concurrent_tasks(max_concurrent_tasks: usize) -> usize {
    max_concurrent_tasks.max(1)
}

async fn reserve_task_slot(
    processing: &Arc<Mutex<HashSet<String>>>,
    task_id: &str,
    max_concurrent_tasks: usize,
) -> TaskReservation {
    let mut proc = processing.lock().await;
    if proc.contains(task_id) {
        TaskReservation::AlreadyProcessing
    } else if proc.len() >= max_concurrent_tasks {
        TaskReservation::AtCapacity
    } else {
        proc.insert(task_id.to_string());
        TaskReservation::Reserved
    }
}

fn export_worker_runtime_env(server: &str, token: &Option<String>, worker_id: &str) {
    // SAFETY: The worker sets these process-wide variables once during startup before
    // spawning Git helper child processes. They are required so Git credential helpers
    // invoked by later shell/git commands can reach the control plane securely.
    unsafe {
        std::env::set_var("CODETETHER_SERVER", server);
        std::env::set_var("CODETETHER_WORKER_ID", worker_id);
        if let Some(token) = token {
            std::env::set_var("CODETETHER_TOKEN", token);
        }
    }
}

fn advertised_interfaces(public_url: Option<&str>) -> serde_json::Value {
    let Some(base_url) = public_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.trim_end_matches('/').to_string())
    else {
        return serde_json::json!({});
    };

    serde_json::json!({
        "http": {
            "base_url": base_url,
        },
        "bus": {
            "stream_url": format!("{base_url}/v1/bus/stream"),
            "publish_url": format!("{base_url}/v1/bus/publish"),
        },
    })
}

/// Approval policy used by the worker when deciding whether to execute tools automatically.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker::AutoApprove;
///
/// let mode = AutoApprove::Safe;
/// assert!(matches!(mode, AutoApprove::Safe));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum AutoApprove {
    All,
    Safe,
    None,
}

/// Default A2A server URL when none is configured
pub const DEFAULT_A2A_SERVER_URL: &str = "https://api.codetether.run";

/// Capabilities of the codetether-agent worker
const BASE_WORKER_CAPABILITIES: &[&str] = &[
    "forage", "ralph", "swarm", "rlm", "a2a", "mcp", "grpc", "grpc-web", "jsonrpc",
];

fn worker_capabilities() -> Vec<String> {
    let mut capabilities: Vec<String> = BASE_WORKER_CAPABILITIES
        .iter()
        .map(|capability| capability.to_string())
        .collect();

    let is_knative = std::env::var("KNATIVE_SERVICE")
        .map(|value| {
            let normalized = value.trim().to_lowercase();
            normalized == "1" || normalized == "true" || normalized == "yes"
        })
        .unwrap_or(false);
    if is_knative {
        capabilities.push("knative".to_string());
    }

    capabilities
}

/// Copy timeline progress into the runtime's shared progress state for heartbeat.
async fn sync_timeline_to_runtime(
    timeline: &task_timeline::TaskTimeline,
    runtime: &WorkerTaskRuntime,
) {
    timeline.sync_progress().await;
    let state = timeline.progress_handle().lock().await.clone();
    *runtime.task_progress.lock().await = state;
}

/// Resolve workspace IDs for the configured workspace roots and log diagnostics.
async fn resolve_and_log_workspace_ids(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_roots: &[String],
) -> Vec<String> {
    match workspace_resolve::resolve_workspace_ids(client, server, token, workspace_roots).await {
        Ok(resolved) => {
            if resolved.is_empty() {
                tracing::warn!(
                    roots = ?workspace_roots,
                    "No server-side workspace IDs resolved for configured roots \
                     — the server may not route tasks to this worker. \
                     Ensure workspaces are registered with the control plane \
                     and that their paths fall under the configured roots."
                );
            } else {
                let ids: Vec<String> = resolved.iter().map(|ws| ws.id.clone()).collect();
                tracing::info!(
                    workspace_ids = ?ids,
                    count = resolved.len(),
                    "Resolved server-side workspace IDs for configured roots"
                );
            }
            resolved.iter().map(|ws| ws.id.clone()).collect()
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to resolve workspace IDs from server");
            Vec::new()
        }
    }
}

fn task_value<'a>(task: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    task.get("task")
        .and_then(|t| t.get(key))
        .or_else(|| task.get(key))
}

fn task_str<'a>(task: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    task_value(task, key).and_then(|v| v.as_str())
}

fn task_metadata(task: &serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    task_value(task, "metadata")
        .and_then(|m| m.as_object())
        .cloned()
        .unwrap_or_default()
}

fn model_ref_to_provider_model(model: &str) -> String {
    // Convert "provider:model" to "provider/model" format, but only if
    // there is no '/' already present. Model IDs like "amazon.nova-micro-v1:0"
    // contain colons as version separators and must NOT be converted.
    if !model.contains('/') && model.contains(':') {
        model.replacen(':', "/", 1)
    } else {
        model.to_string()
    }
}

fn is_swarm_agent(agent_type: &str) -> bool {
    matches!(
        agent_type.trim().to_ascii_lowercase().as_str(),
        "swarm" | "parallel" | "multi-agent"
    )
}

fn is_forage_agent(agent_type: &str) -> bool {
    agent_type.trim().eq_ignore_ascii_case("forage")
}

fn metadata_lookup<'a>(
    metadata: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<&'a serde_json::Value> {
    metadata
        .get(key)
        .or_else(|| {
            metadata
                .get("routing")
                .and_then(|v| v.as_object())
                .and_then(|obj| obj.get(key))
        })
        .or_else(|| {
            metadata
                .get("swarm")
                .and_then(|v| v.as_object())
                .and_then(|obj| obj.get(key))
        })
        .or_else(|| {
            metadata
                .get("forage")
                .and_then(|v| v.as_object())
                .and_then(|obj| obj.get(key))
        })
}

fn metadata_str(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<String> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key).and_then(|v| v.as_str()) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn metadata_usize(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<usize> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_u64() {
                return usize::try_from(v).ok();
            }
            if let Some(v) = value.as_i64()
                && v >= 0
            {
                return usize::try_from(v as u64).ok();
            }
            if let Some(v) = value.as_str()
                && let Ok(parsed) = v.trim().parse::<usize>()
            {
                return Some(parsed);
            }
        }
    }
    None
}

fn metadata_u64(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<u64> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_u64() {
                return Some(v);
            }
            if let Some(v) = value.as_i64()
                && v >= 0
            {
                return Some(v as u64);
            }
            if let Some(v) = value.as_str()
                && let Ok(parsed) = v.trim().parse::<u64>()
            {
                return Some(parsed);
            }
        }
    }
    None
}

fn metadata_bool(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<bool> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_bool() {
                return Some(v);
            }
            if let Some(v) = value.as_str() {
                match v.trim().to_ascii_lowercase().as_str() {
                    "1" | "true" | "yes" | "on" => return Some(true),
                    "0" | "false" | "no" | "off" => return Some(false),
                    _ => {}
                }
            }
        }
    }
    None
}

fn metadata_f64(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<f64> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_f64() {
                return Some(v);
            }
            if let Some(v) = value.as_i64() {
                return Some(v as f64);
            }
            if let Some(v) = value.as_u64() {
                return Some(v as f64);
            }
            if let Some(v) = value.as_str()
                && let Ok(parsed) = v.trim().parse::<f64>()
            {
                return Some(parsed);
            }
        }
    }
    None
}

fn metadata_string_list(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Vec<String> {
    for key in keys {
        let Some(value) = metadata_lookup(metadata, key) else {
            continue;
        };

        if let Some(items) = value.as_array() {
            let parsed = items
                .iter()
                .filter_map(|item| item.as_str())
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            if !parsed.is_empty() {
                return parsed;
            }
        }

        if let Some(value) = value.as_str() {
            let parsed = value
                .split(['\n', ','])
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            if !parsed.is_empty() {
                return parsed;
            }
        }
    }

    Vec::new()
}

fn normalize_forage_execution_engine(value: Option<String>) -> String {
    match value
        .as_deref()
        .unwrap_or("run")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "swarm" => "swarm".to_string(),
        "go" => "go".to_string(),
        _ => "run".to_string(),
    }
}

fn normalize_forage_swarm_strategy(value: Option<String>) -> String {
    match value
        .as_deref()
        .unwrap_or("auto")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "domain" => "domain".to_string(),
        "data" => "data".to_string(),
        "stage" => "stage".to_string(),
        "none" => "none".to_string(),
        _ => "auto".to_string(),
    }
}

fn build_forage_args(
    prompt: &str,
    title: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
    selected_model: Option<String>,
) -> ForageArgs {
    let mut moonshots = metadata_string_list(
        metadata,
        &["moonshots", "moonshot", "goals", "mission", "missions"],
    );
    if moonshots.is_empty() {
        let fallback = prompt.trim();
        if !fallback.is_empty() {
            moonshots.push(fallback.to_string());
        } else if !title.trim().is_empty() {
            moonshots.push(title.trim().to_string());
        }
    }

    ForageArgs {
        top: metadata_usize(metadata, &["top"]).unwrap_or(3),
        loop_mode: metadata_bool(metadata, &["loop", "loop_mode"]).unwrap_or(false),
        interval_secs: metadata_u64(metadata, &["interval_secs", "interval"]).unwrap_or(120),
        max_cycles: metadata_usize(metadata, &["max_cycles"]).unwrap_or(0),
        execute: metadata_bool(metadata, &["execute"]).unwrap_or(false),
        // Task-queue forage should run without requiring S3 archival by default.
        no_s3: metadata_bool(metadata, &["no_s3", "local_only"]).unwrap_or(true),
        moonshots,
        moonshot_file: metadata_str(metadata, &["moonshot_file"]).map(PathBuf::from),
        moonshot_required: metadata_bool(metadata, &["moonshot_required"]).unwrap_or(false),
        moonshot_min_alignment: metadata_f64(metadata, &["moonshot_min_alignment"]).unwrap_or(0.10),
        execution_engine: normalize_forage_execution_engine(metadata_str(
            metadata,
            &["execution_engine", "engine"],
        )),
        run_timeout_secs: metadata_u64(metadata, &["run_timeout_secs", "timeout_secs", "timeout"])
            .unwrap_or(900),
        fail_fast: metadata_bool(metadata, &["fail_fast"]).unwrap_or(false),
        swarm_strategy: normalize_forage_swarm_strategy(metadata_str(
            metadata,
            &["swarm_strategy", "strategy"],
        )),
        swarm_max_subagents: metadata_usize(metadata, &["swarm_max_subagents"]).unwrap_or(8),
        swarm_max_steps: metadata_usize(metadata, &["swarm_max_steps"]).unwrap_or(100),
        swarm_subagent_timeout_secs: metadata_u64(metadata, &["swarm_subagent_timeout_secs"])
            .unwrap_or(300),
        model: selected_model,
        json: metadata_bool(metadata, &["json"]).unwrap_or(false),
    }
}

fn parse_swarm_strategy(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> DecompositionStrategy {
    match metadata_str(
        metadata,
        &[
            "decomposition_strategy",
            "swarm_strategy",
            "strategy",
            "swarm_decomposition",
        ],
    )
    .as_deref()
    .map(|s| s.to_ascii_lowercase())
    .as_deref()
    {
        Some("none") | Some("single") => DecompositionStrategy::None,
        Some("domain") | Some("by_domain") => DecompositionStrategy::ByDomain,
        Some("data") | Some("by_data") => DecompositionStrategy::ByData,
        Some("stage") | Some("by_stage") => DecompositionStrategy::ByStage,
        _ => DecompositionStrategy::Automatic,
    }
}

async fn resolve_swarm_model(
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> Option<String> {
    if let Some(model) = explicit_model
        && !model.trim().is_empty()
    {
        return Some(model);
    }

    let registry = ProviderRegistry::from_vault().await.ok()?;
    let providers = registry.list();
    if providers.is_empty() {
        return None;
    }
    let provider = choose_provider_for_tier(providers.as_slice(), model_tier);
    let model = default_model_for_provider(provider, model_tier);
    Some(format!("{}/{}", provider, model))
}

fn format_swarm_event_for_output(event: &SwarmEvent) -> Option<String> {
    match event {
        SwarmEvent::Started {
            task,
            total_subtasks,
        } => Some(format!(
            "[swarm] started task={} planned_subtasks={}",
            task, total_subtasks
        )),
        SwarmEvent::StageComplete {
            stage,
            completed,
            failed,
        } => Some(format!(
            "[swarm] stage={} completed={} failed={}",
            stage, completed, failed
        )),
        SwarmEvent::SubTaskUpdate { id, status, .. } => Some(format!(
            "[swarm] subtask id={} status={}",
            &id.chars().take(8).collect::<String>(),
            format!("{status:?}").to_ascii_lowercase()
        )),
        SwarmEvent::AgentToolCall {
            subtask_id,
            tool_name,
        } => Some(format!(
            "[swarm] subtask id={} tool={}",
            &subtask_id.chars().take(8).collect::<String>(),
            tool_name
        )),
        SwarmEvent::AgentError { subtask_id, error } => Some(format!(
            "[swarm] subtask id={} error={}",
            &subtask_id.chars().take(8).collect::<String>(),
            error
        )),
        SwarmEvent::Complete { success, stats } => Some(format!(
            "[swarm] complete success={} subtasks={} speedup={:.2}",
            success,
            stats.subagents_completed + stats.subagents_failed,
            stats.speedup_factor
        )),
        SwarmEvent::Error(err) => Some(format!("[swarm] error message={}", err)),
        _ => None,
    }
}

pub async fn register_worker(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    public_url: Option<&str>,
) -> Result<()> {
    // Load ProviderRegistry and collect available models
    let models = match load_provider_models().await {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!(
                "Failed to load provider models: {}, proceeding without model info",
                e
            );
            HashMap::new()
        }
    };

    // Register via the workers/register endpoint
    let mut req = client.post(format!("{}/v1/agent/workers/register", server));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    // Flatten models HashMap into array of model objects with pricing data
    // matching the format expected by the A2A server's /models and /workers endpoints
    let models_array: Vec<serde_json::Value> = models
        .iter()
        .flat_map(|(provider, model_infos)| {
            model_infos.iter().map(move |m| {
                let mut obj = serde_json::json!({
                    "id": format!("{}/{}", provider, m.id),
                    "name": &m.id,
                    "provider": provider,
                    "provider_id": provider,
                });
                if let Some(input_cost) = m.input_cost_per_million {
                    obj["input_cost_per_million"] = serde_json::json!(input_cost);
                }
                if let Some(output_cost) = m.output_cost_per_million {
                    obj["output_cost_per_million"] = serde_json::json!(output_cost);
                }
                obj
            })
        })
        .collect();

    tracing::info!(
        "Registering worker with {} models from {} providers",
        models_array.len(),
        models.len()
    );

    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let k8s_node_name = std::env::var("K8S_NODE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    // Collect agent definitions from the builtin registry
    let registry = crate::agent::AgentRegistry::with_builtins();
    let agent_defs: Vec<serde_json::Value> = registry
        .list()
        .iter()
        .map(|info| {
            serde_json::json!({
                "name": info.name,
                "description": info.description,
                "mode": format!("{:?}", info.mode).to_lowercase(),
                "native": info.native,
                "hidden": info.hidden,
                "model": info.model,
                "temperature": info.temperature,
                "top_p": info.top_p,
                "max_steps": info.max_steps,
            })
        })
        .collect();

    // Resolve workspace IDs that correspond to our configured paths.
    let workspace_ids = resolve_and_log_workspace_ids(client, server, token, codebases).await;

    let res = req
        .json(&serde_json::json!({
            "worker_id": worker_id,
            "name": name,
            "capabilities": worker_capabilities(),
            "hostname": hostname,
            "k8s_node_name": k8s_node_name,
            "models": models_array,
            "workspaces": codebases,
            "workspace_ids": workspace_ids,
            "interfaces": advertised_interfaces(public_url),
            "agents": agent_defs,
        }))
        .send()
        .await?;

    if res.status().is_success() {
        tracing::info!("Worker registered successfully");
    } else {
        tracing::warn!("Failed to register worker: {}", res.status());
    }

    Ok(())
}

/// Load ProviderRegistry and collect all available models grouped by provider.
/// Tries Vault first, then falls back to config/env vars if Vault is unreachable.
/// Returns ModelInfo structs (with pricing data when available).
///
/// Result is cached process-wide: repeated calls (e.g. TUI startup +
/// background worker registration) do not re-hit every provider's
/// `/models` endpoint. The cache is populated exactly once per process
/// on the first call, so concurrent callers share one network fanout.
async fn load_provider_models() -> Result<HashMap<String, Vec<crate::provider::ModelInfo>>> {
    use tokio::sync::OnceCell;
    static CACHE: OnceCell<HashMap<String, Vec<crate::provider::ModelInfo>>> =
        OnceCell::const_new();
    CACHE
        .get_or_try_init(|| async { load_provider_models_uncached().await })
        .await
        .cloned()
}

async fn load_provider_models_uncached() -> Result<HashMap<String, Vec<crate::provider::ModelInfo>>>
{
    // Try Vault first
    let registry = match ProviderRegistry::from_vault().await {
        Ok(r) if !r.list().is_empty() => {
            tracing::info!("Loaded {} providers from Vault", r.list().len());
            r
        }
        Ok(_) => {
            tracing::warn!("Vault returned 0 providers, falling back to config/env vars");
            fallback_registry().await?
        }
        Err(e) => {
            tracing::warn!("Vault unreachable ({}), falling back to config/env vars", e);
            fallback_registry().await?
        }
    };

    let mut models_by_provider: HashMap<String, Vec<crate::provider::ModelInfo>> = HashMap::new();

    for provider_name in registry.list() {
        if let Some(provider) = registry.get(provider_name) {
            match provider.list_models().await {
                Ok(models) => {
                    if !models.is_empty() {
                        tracing::debug!("Provider {}: {} models", provider_name, models.len());
                        models_by_provider.insert(provider_name.to_string(), models);
                    }
                }
                Err(e) => {
                    tracing::debug!("Failed to list models for {}: {}", provider_name, e);
                }
            }
        }
    }

    Ok(models_by_provider)
}

/// Fallback: build a ProviderRegistry from config file + environment variables
async fn fallback_registry() -> Result<ProviderRegistry> {
    let config = crate::config::Config::load().await.unwrap_or_default();
    ProviderRegistry::from_config(&config).await
}

async fn fetch_pending_tasks(runtime: &WorkerTaskRuntime) -> Result<()> {
    tracing::info!("Checking for pending tasks...");

    let mut req = runtime
        .client
        .get(format!("{}/v1/agent/tasks?status=pending", runtime.server));
    if let Some(t) = &runtime.token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        return Ok(());
    }

    let data: serde_json::Value = res.json().await?;
    // Handle both plain array response and {tasks: [...]} wrapper
    let tasks = if let Some(arr) = data.as_array() {
        arr.clone()
    } else {
        data["tasks"].as_array().cloned().unwrap_or_default()
    };

    tracing::info!("Found {} pending task(s)", tasks.len());

    for task in tasks {
        if let Some(id) = task["id"].as_str() {
            // Scope gate: reject tasks not targeted at this worker
            if let Err(reason) = check_task_scope(&task, &runtime.worker_id, &runtime.agent_name, &runtime.workspace_ids) {
                tracing::debug!(
                    task_id = id,
                    reason = %reason,
                    "Pending task skipped — out of scope"
                );
                continue;
            }

            match reserve_task_slot(&runtime.processing, id, runtime.max_concurrent_tasks).await {
                TaskReservation::Reserved => {
                    let task_id = id.to_string();
                    let runtime = runtime.clone();

                    tokio::spawn(async move {
                        if let Err(e) = handle_task(&runtime, &task).await {
                            tracing::error!("Task {} failed: {}", task_id, e);
                        }
                        runtime.processing.lock().await.remove(&task_id);
                    });
                }
                TaskReservation::AlreadyProcessing => {}
                TaskReservation::AtCapacity => {
                    tracing::debug!(
                        max_concurrent_tasks = runtime.max_concurrent_tasks,
                        "Worker is at task capacity; leaving remaining tasks pending"
                    );
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn connect_stream(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    task_notify_rx: Option<mpsc::Receiver<String>>,
) -> Result<()> {
    let url = format!(
        "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
        runtime.server,
        urlencoding::encode(name),
        urlencoding::encode(&runtime.worker_id)
    );

    let mut req = runtime
        .client
        .get(&url)
        .header("Accept", "text/event-stream")
        .header("X-Worker-ID", &runtime.worker_id)
        .header("X-Agent-Name", name)
        .header("X-Codebases", codebases.join(","))
        .header("X-Workspaces", codebases.join(","));

    if let Some(t) = &runtime.token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        anyhow::bail!("Failed to connect: {}", res.status());
    }

    tracing::info!("Connected to A2A server");

    let mut stream = res.bytes_stream();
    let mut buffer = String::new();
    let mut poll_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    poll_interval.tick().await; // consume the initial immediate tick

    // Pin the optional receiver so we can use it in the loop
    let mut task_notify_rx = task_notify_rx;

    loop {
        tokio::select! {
            // Handle task notification from CloudEvent (Knative Eventing)
            // Only if the channel was provided (i.e., running with HTTP server)
            task_id = async {
                if let Some(ref mut rx) = task_notify_rx {
                    rx.recv().await
                } else {
                    // Never ready when None - use pending to skip this branch
                    futures::future::pending().await
                }
            } => {
                if let Some(task_id) = task_id {
                    tracing::info!("Received task notification via CloudEvent: {}", task_id);
                    // Immediately poll for and process this task
                    if let Err(e) = poll_pending_tasks(runtime).await {
                        tracing::warn!("Task notification poll failed: {}", e);
                    }
                }
            }
            chunk = stream.next() => {
                match chunk {
                    Some(Ok(chunk)) => {
                        buffer.push_str(&String::from_utf8_lossy(&chunk));

                        // Process SSE events
                        while let Some(pos) = buffer.find("\n\n") {
                            let event_str = buffer[..pos].to_string();
                            buffer = buffer[pos + 2..].to_string();

                            if let Some(data_line) = event_str.lines().find(|l| l.starts_with("data:")) {
                                let data = data_line.trim_start_matches("data:").trim();
                                if data == "[DONE]" || data.is_empty() {
                                    continue;
                                }

                                if let Ok(task) = serde_json::from_str::<serde_json::Value>(data) {
                                    spawn_task_handler(&task, runtime).await;
                                }
                            }
                        }
                    }
                    Some(Err(e)) => {
                        return Err(e.into());
                    }
                    None => {
                        // Stream ended
                        return Ok(());
                    }
                }
            }
            _ = poll_interval.tick() => {
                // Periodic poll for pending tasks the SSE stream may have missed
                if let Err(e) = poll_pending_tasks(runtime).await {
                    tracing::warn!("Periodic task poll failed: {}", e);
                }
            }
        }
    }
}

async fn spawn_task_handler(task: &serde_json::Value, runtime: &WorkerTaskRuntime) {
    if let Some(id) = task
        .get("task")
        .and_then(|t| t["id"].as_str())
        .or_else(|| task["id"].as_str())
    {
        // Scope gate: reject tasks not targeted at this worker
        if let Err(reason) = check_task_scope(task, &runtime.worker_id, &runtime.agent_name, &runtime.workspace_ids) {
            tracing::debug!(
                task_id = id,
                reason = %reason,
                "SSE task skipped — out of scope"
            );
            return;
        }

        match reserve_task_slot(&runtime.processing, id, runtime.max_concurrent_tasks).await {
            TaskReservation::Reserved => {
                let task_id = id.to_string();
                let task = task.clone();
                let runtime = runtime.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_task(&runtime, &task).await {
                        tracing::error!("Task {} failed: {}", task_id, e);
                    }
                    runtime.processing.lock().await.remove(&task_id);
                });
            }
            TaskReservation::AlreadyProcessing => {}
            TaskReservation::AtCapacity => {
                tracing::debug!(
                    task_id = id,
                    max_concurrent_tasks = runtime.max_concurrent_tasks,
                    "Worker is at task capacity; task will stay pending until a slot frees up"
                );
            }
        }
    }
}

async fn poll_pending_tasks(runtime: &WorkerTaskRuntime) -> Result<()> {
    let mut req = runtime
        .client
        .get(format!("{}/v1/agent/tasks?status=pending", runtime.server));
    if let Some(t) = &runtime.token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        return Ok(());
    }

    let data: serde_json::Value = res.json().await?;
    let tasks = if let Some(arr) = data.as_array() {
        arr.clone()
    } else {
        data["tasks"].as_array().cloned().unwrap_or_default()
    };

    if !tasks.is_empty() {
        tracing::debug!("Poll found {} pending task(s)", tasks.len());
    }

    for task in &tasks {
        // Scope gate: reject tasks not targeted at this worker
        let task_id_str = task_str(task, "id").unwrap_or("?");
        if let Err(reason) = check_task_scope(task, &runtime.worker_id, &runtime.agent_name, &runtime.workspace_ids) {
            tracing::debug!(
                task_id = task_id_str,
                reason = %reason,
                "Poll task skipped — out of scope"
            );
            continue;
        }
        spawn_task_handler(task, runtime).await;
    }

    Ok(())
}

async fn handle_task(runtime: &WorkerTaskRuntime, task: &serde_json::Value) -> Result<()> {
    let task_id = task_str(task, "id").ok_or_else(|| anyhow::anyhow!("No task ID"))?;
    let title = task_str(task, "title").unwrap_or("Untitled");

    // Determine task timeout from metadata or default to 1200s (Knative default).
    let metadata = task_metadata(task);
    let timeout_secs = metadata_u64(&metadata, &["timeout_secs", "timeout"])
        .unwrap_or(1200)
        .clamp(60, 3600);

    let mut timeline = task_timeline::TaskTimeline::new(task_id, timeout_secs);
    timeline.checkpoint(task_timeline::TaskCheckpoint::TaskReceived);
    tracing::info!(
        "Handling task: {} ({}) [timeout={}s]",
        title,
        task_id,
        timeout_secs
    );

    // Wire the timeline's progress state into the runtime so the heartbeat picks it up.
    // We'll sync after each checkpoint.
    sync_timeline_to_runtime(&timeline, runtime).await;

    // Claim the task
    timeline.checkpoint(task_timeline::TaskCheckpoint::ClaimRequested);
    let mut req = runtime
        .client
        .post(format!("{}/v1/worker/tasks/claim", runtime.server))
        .header("X-Worker-ID", &runtime.worker_id);
    if let Some(t) = &runtime.token {
        req = req.bearer_auth(t);
    }

    let res = req
        .json(&serde_json::json!({ "task_id": task_id }))
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await?;
        if status == reqwest::StatusCode::CONFLICT {
            tracing::debug!(task_id, "Task already claimed by another worker, skipping");
        } else {
            tracing::warn!(task_id, %status, "Failed to claim task: {}", text);
        }
        return Ok(());
    }

    let claim = res.json::<TaskClaimResponse>().await?;
    let provider_keys = claim.provider_keys.clone();
    let claim_provenance = claim.into_provenance();
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::Claimed,
        Some(format!(
            "run_id={:?} attempt_id={:?}",
            claim_provenance.run_id, claim_provenance.attempt_id
        )),
    );
    tracing::info!(
        task_id,
        run_id = ?claim_provenance.run_id,
        attempt_id = ?claim_provenance.attempt_id,
        "Claimed task"
    );

    // --- All post-claim work is wrapped so we always release the task ---
    let inner_result: Result<(&str, Option<String>, Option<String>, Option<String>)> =
        execute_claimed_task(
            runtime,
            task,
            task_id,
            title,
            &claim_provenance,
            provider_keys,
            &mut timeline,
        )
        .await;

    let (status, result, error, session_id) = match inner_result {
        Ok(tuple) => tuple,
        Err(e) => {
            tracing::error!(
                task_id,
                error = %e,
                "Task failed after claim (releasing as failed)"
            );
            timeline.checkpoint_with_detail(
                task_timeline::TaskCheckpoint::Failed,
                Some(format!("{}", e)),
            );
            (
                "failed",
                None,
                Some(format!("Worker error after claim: {}", e)),
                None,
            )
        }
    };

    // Emit final diagnostics before releasing
    timeline.emit_diagnostics();
    let diagnostics_json = timeline.diagnostics_json();

    timeline.checkpoint(task_timeline::TaskCheckpoint::Releasing);
    release_task_result(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        &runtime.worker_id,
        task_id,
        status,
        result,
        error,
        session_id,
        Some(diagnostics_json),
    )
    .await?;

    timeline.checkpoint(task_timeline::TaskCheckpoint::Released);

    // Clear task progress so heartbeat stops reporting stale state
    runtime.task_progress.lock().await.clear();

    tracing::info!("Task released: {} with status: {}", task_id, status);
    Ok(())
}

/// Inner logic for a claimed task. Returns (status, result, error, session_id).
/// Extracted so that `handle_task` can catch any error and always release.
#[allow(clippy::too_many_lines)]
async fn execute_claimed_task<'a>(
    runtime: &WorkerTaskRuntime,
    task: &'a serde_json::Value,
    task_id: &'a str,
    title: &'a str,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<(&'static str, Option<String>, Option<String>, Option<String>)> {
    let metadata = task_metadata(task);
    timeline.checkpoint(task_timeline::TaskCheckpoint::MetadataParsed);
    let resume_session_id = metadata
        .get("resume_session_id")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let complexity_hint = metadata_str(&metadata, &["complexity"]);
    let model_tier = metadata_str(&metadata, &["model_tier", "tier"])
        .map(|s| s.to_ascii_lowercase())
        .or_else(|| {
            complexity_hint.as_ref().map(|complexity| {
                match complexity.to_ascii_lowercase().as_str() {
                    "quick" => "fast".to_string(),
                    "deep" => "heavy".to_string(),
                    _ => "balanced".to_string(),
                }
            })
        });
    let worker_personality = metadata_str(
        &metadata,
        &["worker_personality", "personality", "agent_personality"],
    );
    let target_agent_name = metadata_str(&metadata, &["target_agent_name", "agent_name"]);
    let raw_model = task_str(task, "model_ref")
        .or_else(|| metadata_lookup(&metadata, "model_ref").and_then(|v| v.as_str()))
        .or_else(|| task_str(task, "model"))
        .or_else(|| metadata_lookup(&metadata, "model").and_then(|v| v.as_str()));
    let selected_model = raw_model.map(model_ref_to_provider_model);
    let raw_agent = task_str(task, "agent_type")
        .or_else(|| task_str(task, "agent"))
        .unwrap_or("build");
    let prompt = task_str(task, "prompt")
        .or_else(|| task_str(task, "description"))
        .unwrap_or(title);

    // Detect virtual/global workspace tasks dispatched via the API.
    let workspace_id = task_str(task, "workspace_id")
        .or_else(|| metadata_lookup(&metadata, "workspace_id").and_then(|v| v.as_str()));
    let is_virtual_task = workspace_id.map_or(false, |ws| ws == "global" || ws.is_empty());

    if raw_agent.eq_ignore_ascii_case("clone_repo") {
        return match handle_clone_repo_task(
            &runtime.client,
            &runtime.server,
            &runtime.token,
            &runtime.worker_id,
            task,
            &metadata,
        )
        .await
        {
            Ok(message) => Ok(("completed", Some(message), None, None)),
            Err(err) => {
                tracing::error!(task_id, error = %err, "Clone task failed");
                Ok(("failed", None, Some(format!("Error: {}", err)), None))
            }
        };
    }

    if is_forage_agent(raw_agent) {
        return match handle_forage_task(title, prompt, &metadata, selected_model.clone()).await {
            Ok(message) => Ok(("completed", Some(message), None, None)),
            Err(err) => {
                tracing::error!(task_id, error = %err, "Forage task failed");
                Ok(("failed", None, Some(format!("Error: {}", err)), None))
            }
        };
    }

    // Resume existing session when requested; fall back to a fresh session if missing.
    let mut session = if let Some(ref sid) = resume_session_id {
        match Session::load(sid).await {
            Ok(existing) => {
                tracing::info!("Resuming session {} for task {}", sid, task_id);
                existing
            }
            Err(e) => {
                tracing::warn!(
                    "Could not load session {} for task {} ({}), starting a new session",
                    sid,
                    task_id,
                    e
                );
                Session::new().await?
            }
        }
    } else {
        Session::new().await?
    };
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::SessionReady,
        Some(format!("session_id={}", session.id)),
    );

    if !is_virtual_task {
        if let Some(workspace_path) = resolve_task_workspace_dir(
            &runtime.client,
            &runtime.server,
            &runtime.token,
            workspace_id,
        )
        .await?
        {
            session.metadata.directory = Some(workspace_path);
        }
        timeline.checkpoint_with_detail(
            task_timeline::TaskCheckpoint::WorkspaceReady,
            session
                .metadata
                .directory
                .as_deref()
                .map(|d| d.display().to_string()),
        );
    }

    // For virtual/global tasks, clear the workspace directory so tools don't
    // try to operate on a workspace that doesn't exist on this worker.
    if is_virtual_task {
        tracing::info!(
            task_id,
            workspace_id = workspace_id.unwrap_or("(none)"),
            "Virtual task detected — skipping workspace setup"
        );
        session.metadata.directory = None;
    }

    // Normalize agent: only "build", "plan", and swarm types are valid.
    // Map deprecated/unknown agent types (e.g. "general", "explore") to "build".
    let agent_type = if is_swarm_agent(raw_agent) {
        raw_agent
    } else {
        match raw_agent {
            "build" | "plan" => raw_agent,
            other => {
                tracing::info!(
                    "Agent \"{}\" is not a primary agent, falling back to \"build\"",
                    other
                );
                "build"
            }
        }
    };
    session.set_agent_name(agent_type.to_string());
    session.attach_claim_provenance(claim_provenance);
    session.metadata.provider_keys = provider_keys;

    // Skip git hook installation for virtual tasks (no workspace directory).
    if !is_virtual_task {
        if let Some(directory) = session.metadata.directory.as_deref()
            && let Err(err) = install_commit_msg_hook(directory)
        {
            tracing::warn!(task_id, error = %err, "Failed to install commit-msg hook");
        }
        timeline.checkpoint(task_timeline::TaskCheckpoint::GitHookInstalled);
    }

    if let Some(model) = selected_model.clone() {
        session.metadata.model = Some(model);
    }
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::ModelSelected,
        session.metadata.model.clone(),
    );

    tracing::info!(task_id, agent_type, "Executing prompt: {}", prompt);
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::AgentStarting,
        Some(format!(
            "agent={} model={:?}",
            agent_type, session.metadata.model
        )),
    );
    sync_timeline_to_runtime(&timeline, runtime).await;

    // Set up output streaming to forward progress to the server and bus
    let stream_client = runtime.client.clone();
    let stream_server = runtime.server.clone();
    let stream_token = runtime.token.clone();
    let stream_worker_id = runtime.worker_id.clone();
    let stream_task_id = task_id.to_string();
    let stream_bus = Arc::clone(&runtime.bus);

    let output_callback: Arc<dyn Fn(String) + Send + Sync + 'static> =
        Arc::new(move |output: String| {
            let c = stream_client.clone();
            let s = stream_server.clone();
            let t = stream_token.clone();
            let w = stream_worker_id.clone();
            let tid = stream_task_id.clone();

            let bus_handle = stream_bus.handle("task-output");
            bus_handle.send(
                format!("task.{}", tid),
                crate::bus::BusMessage::TaskUpdate {
                    task_id: tid.clone(),
                    state: crate::a2a::types::TaskState::Working,
                    message: Some(output.clone()),
                },
            );

            tokio::spawn(async move {
                let mut req = c
                    .post(format!("{}/v1/agent/tasks/{}/output", s, tid))
                    .header("X-Worker-ID", &w);
                if let Some(tok) = &t {
                    req = req.bearer_auth(tok);
                }
                let _ = req
                    .json(&serde_json::json!({
                        "worker_id": w,
                        "output": output,
                    }))
                    .send()
                    .await;
            });
        });

    // Execute swarm tasks via SwarmExecutor; all other agents use the standard session loop.
    let (status, result, error, session_id) = if is_swarm_agent(agent_type) {
        match execute_swarm_with_policy(
            &mut session,
            prompt,
            model_tier.as_deref(),
            selected_model,
            &metadata,
            complexity_hint.as_deref(),
            worker_personality.as_deref(),
            target_agent_name.as_deref(),
            Some(&runtime.bus),
            Some(Arc::clone(&output_callback)),
        )
        .await
        {
            Ok((session_result, true)) => {
                tracing::info!("Swarm task completed successfully: {}", task_id);
                (
                    "completed",
                    Some(session_result.text),
                    None,
                    Some(session_result.session_id),
                )
            }
            Ok((session_result, false)) => {
                tracing::warn!("Swarm task completed with failures: {}", task_id);
                (
                    "failed",
                    Some(session_result.text),
                    Some("Swarm execution completed with failures".to_string()),
                    Some(session_result.session_id),
                )
            }
            Err(e) => {
                tracing::error!("Swarm task failed: {} - {}", task_id, e);
                ("failed", None, Some(format!("Error: {}", e)), None)
            }
        }
    } else {
        match execute_session_with_policy(
            &mut session,
            prompt,
            runtime.auto_approve,
            model_tier.as_deref(),
            Some(Arc::clone(&output_callback)),
        )
        .await
        {
            Ok(session_result) => {
                tracing::info!("Task completed successfully: {}", task_id);
                (
                    "completed",
                    Some(session_result.text),
                    None,
                    Some(session_result.session_id),
                )
            }
            Err(e) => {
                tracing::error!(task_id, error = %e, "Task execution failed");
                ("failed", None, Some(format!("Error: {}", e)), None)
            }
        }
    };

    // Record agent completion checkpoint
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::AgentDone,
        Some(format!("status={}", status)),
    );
    sync_timeline_to_runtime(&timeline, runtime).await;

    // Check deadline before proceeding to commit/release phase
    if timeline.is_near_deadline() {
        tracing::warn!(
            task_id,
            elapsed_secs = format!("{:.1}", timeline.elapsed_secs()),
            budget_pct = format!("{:.1}%", timeline.budget_pct_used()),
            "Near deadline after agent completion — will attempt graceful shutdown"
        );
        timeline.checkpoint(task_timeline::TaskCheckpoint::GracefulShutdown);
    }

    // Sync progress state for heartbeat reporting
    timeline.sync_progress().await;

    Ok((
        status,
        result,
        error,
        Some(session_id.unwrap_or_else(|| session.id.clone())),
    ))
}

async fn handle_forage_task(
    title: &str,
    prompt: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
    selected_model: Option<String>,
) -> Result<String> {
    let forage_args = build_forage_args(prompt, title, metadata, selected_model);
    let summary = crate::forage::execute_with_summary(forage_args).await?;
    Ok(summary.render_text())
}

async fn handle_clone_repo_task(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Result<String> {
    let workspace_id = metadata_str(metadata, &["workspace_id"])
        .or_else(|| task_str(task, "workspace_id").map(|value| value.to_string()))
        .ok_or_else(|| anyhow::anyhow!("Clone task is missing workspace_id metadata"))?;
    let workspace = fetch_workspace_record(client, server, token, &workspace_id).await?;
    let git_url = metadata_str(metadata, &["git_url"])
        .or_else(|| workspace.git_url.clone())
        .ok_or_else(|| anyhow::anyhow!("Workspace {} is missing git_url", workspace_id))?;
    let branch = metadata_str(metadata, &["git_branch"])
        .or_else(|| workspace.git_branch.clone())
        .unwrap_or_else(|| "main".to_string());
    let repo_path = resolve_workspace_clone_path(&workspace_id, &workspace.path);

    if let Some(parent) = repo_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("Failed to create repo parent {}", parent.display()))?;
    }

    let temp_helper_path =
        git_clone_base_dir().join(format!(".{}-git-credential-helper", workspace_id));
    write_git_credential_helper_script(&temp_helper_path, &workspace_id)?;

    let clone_result = async {
        let git_dir = repo_path.join(".git");
        if git_dir.exists() {
            configure_repo_git_auth(&repo_path, &workspace_id)?;
            configure_repo_git_github_app_from_agent_config(
                &repo_path,
                Some(&serde_json::Value::Object(workspace.agent_config.clone())),
            );
            run_git_command_at(
                Some(&repo_path),
                vec!["fetch".to_string(), "origin".to_string(), branch.clone()],
            )
            .await?;
            run_git_command_at(
                Some(&repo_path),
                vec!["checkout".to_string(), branch.clone()],
            )
            .await?;
            run_git_command_at(
                Some(&repo_path),
                vec![
                    "pull".to_string(),
                    "--ff-only".to_string(),
                    "origin".to_string(),
                    branch.clone(),
                ],
            )
            .await?;
        } else {
            prepare_clone_target(&repo_path).await?;

            run_git_command_at(
                None,
                vec![
                    "-c".to_string(),
                    format!("credential.helper={}", temp_helper_path.display()),
                    "-c".to_string(),
                    "credential.useHttpPath=true".to_string(),
                    "clone".to_string(),
                    "--single-branch".to_string(),
                    "--branch".to_string(),
                    branch.clone(),
                    git_url.clone(),
                    repo_path.display().to_string(),
                ],
            )
            .await?;
            configure_repo_git_auth(&repo_path, &workspace_id)?;
            configure_repo_git_github_app_from_agent_config(
                &repo_path,
                Some(&serde_json::Value::Object(workspace.agent_config.clone())),
            );
        }
        install_commit_msg_hook(&repo_path)?;

        register_cloned_workspace(client, server, token, worker_id, &workspace, &repo_path).await?;
        enqueue_post_clone_task(client, server, token, worker_id, &workspace_id, metadata).await?;

        Ok::<String, anyhow::Error>(format!(
            "Repository ready at {} (branch: {})",
            repo_path.display(),
            branch
        ))
    }
    .await;

    let _ = tokio::fs::remove_file(&temp_helper_path).await;
    clone_result
}

async fn register_cloned_workspace(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    workspace: &RegisteredWorkspaceRecord,
    repo_path: &Path,
) -> Result<()> {
    let mut req = client.post(format!(
        "{}/v1/agent/workspaces",
        server.trim_end_matches('/')
    ));
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let response = req
        .json(&serde_json::json!({
            "workspace_id": workspace.id,
            "name": workspace.name,
            "path": repo_path.display().to_string(),
            "description": workspace.description,
            "agent_config": workspace.agent_config,
            "git_url": workspace.git_url,
            "git_branch": workspace.git_branch,
            "worker_id": worker_id,
        }))
        .send()
        .await
        .context("Failed to register cloned workspace with server")?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to register cloned workspace {} ({}): {}",
            workspace.id,
            status,
            body
        );
    }

    Ok(())
}

async fn enqueue_post_clone_task(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    workspace_id: &str,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    let Some(post_clone_task) = metadata.get("post_clone_task") else {
        return Ok(());
    };
    let Some(task) = post_clone_task.as_object() else {
        return Ok(());
    };
    let title = task
        .get("title")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("post_clone_task is missing title"))?;
    let prompt = task
        .get("prompt")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("post_clone_task is missing prompt"))?;
    let agent_type = task
        .get("agent_type")
        .and_then(|value| value.as_str())
        .unwrap_or("build");
    let mut task_metadata = task
        .get("metadata")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(task_metadata_obj) = task_metadata.as_object_mut() {
        task_metadata_obj
            .entry("target_worker_id".to_string())
            .or_insert_with(|| serde_json::Value::String(worker_id.to_string()));
    }

    let mut req = client.post(format!(
        "{}/v1/agent/workspaces/{}/tasks",
        server.trim_end_matches('/'),
        workspace_id
    ));
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let response = req
        .json(&serde_json::json!({
            "title": title,
            "prompt": prompt,
            "agent_type": agent_type,
            "metadata": task_metadata,
        }))
        .send()
        .await
        .context("Failed to enqueue post-clone task")?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Failed to enqueue post-clone task ({}): {}", status, body);
    }
    Ok(())
}

async fn run_git_command_at(current_dir: Option<&Path>, args: Vec<String>) -> Result<String> {
    let mut command = Command::new("git");
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }

    let output = command
        .args(args.iter().map(String::as_str))
        .output()
        .await
        .context("Failed to execute git command")?;

    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }

    Err(anyhow::anyhow!(
        "Git command failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

async fn release_task_result(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task_id: &str,
    status: &str,
    result: Option<String>,
    error: Option<String>,
    session_id: Option<String>,
    diagnostics: Option<serde_json::Value>,
) -> Result<()> {
    let mut req = client
        .post(format!("{}/v1/worker/tasks/release", server))
        .header("X-Worker-ID", worker_id);
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let mut payload = serde_json::json!({
        "task_id": task_id,
        "status": status,
        "result": result,
        "error": error,
        "session_id": session_id,
    });
    if let Some(diagnostics) = diagnostics {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("diagnostics".to_string(), diagnostics);
        }
    }

    req.json(&payload)
        .send()
        .await
        .context("Failed to release task result")?;

    Ok(())
}

async fn execute_swarm_with_policy(
    session: &mut Session,
    prompt: &str,
    model_tier: Option<&str>,
    explicit_model: Option<String>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity_hint: Option<&str>,
    worker_personality: Option<&str>,
    target_agent_name: Option<&str>,
    bus: Option<&Arc<AgentBus>>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<(crate::session::SessionResult, bool)> {
    use crate::provider::{ContentPart, Message, Role};

    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });

    if session.title.is_none() {
        session.generate_title().await?;
    }

    let strategy = parse_swarm_strategy(metadata);
    let max_subagents = metadata_usize(
        metadata,
        &["swarm_max_subagents", "max_subagents", "subagents"],
    )
    .unwrap_or(10)
    .clamp(1, 100);
    let max_steps_per_subagent = metadata_usize(
        metadata,
        &[
            "swarm_max_steps_per_subagent",
            "max_steps_per_subagent",
            "max_steps",
        ],
    )
    .unwrap_or(50)
    .clamp(1, 200);
    let timeout_secs = metadata_u64(metadata, &["swarm_timeout_secs", "timeout_secs", "timeout"])
        .unwrap_or(600)
        .clamp(30, 3600);
    let parallel_enabled =
        metadata_bool(metadata, &["swarm_parallel_enabled", "parallel_enabled"]).unwrap_or(true);

    let model = resolve_swarm_model(explicit_model, model_tier).await;
    if let Some(ref selected_model) = model {
        session.metadata.model = Some(selected_model.clone());
    }

    if let Some(ref cb) = output_callback {
        cb(format!(
            "[swarm] routing complexity={} tier={} personality={} target_agent={}",
            complexity_hint.unwrap_or("standard"),
            model_tier.unwrap_or("balanced"),
            worker_personality.unwrap_or("auto"),
            target_agent_name.unwrap_or("auto")
        ));
        cb(format!(
            "[swarm] config strategy={:?} max_subagents={} max_steps={} timeout={}s tier={}",
            strategy,
            max_subagents,
            max_steps_per_subagent,
            timeout_secs,
            model_tier.unwrap_or("balanced")
        ));
    }

    let swarm_config = SwarmConfig {
        max_subagents,
        max_steps_per_subagent,
        subagent_timeout_secs: timeout_secs,
        parallel_enabled,
        model,
        working_dir: session
            .metadata
            .directory
            .as_ref()
            .map(|p| p.to_string_lossy().to_string()),
        ..Default::default()
    };

    let swarm_result = if output_callback.is_some() {
        let (event_tx, mut event_rx) = mpsc::channel(256);
        let mut executor = SwarmExecutor::new(swarm_config).with_event_tx(event_tx);
        if let Some(bus) = bus {
            executor = executor.with_bus(Arc::clone(bus));
        }
        let prompt_owned = prompt.to_string();
        let mut exec_handle =
            tokio::spawn(async move { executor.execute(&prompt_owned, strategy).await });

        let mut final_result: Option<crate::swarm::SwarmResult> = None;

        while final_result.is_none() {
            tokio::select! {
                maybe_event = event_rx.recv() => {
                    if let Some(event) = maybe_event
                        && let Some(ref cb) = output_callback
                            && let Some(line) = format_swarm_event_for_output(&event) {
                                cb(line);
                            }
                }
                join_result = &mut exec_handle => {
                    let joined = join_result.map_err(|e| anyhow::anyhow!("Swarm join failure: {}", e))?;
                    final_result = Some(joined?);
                }
            }
        }

        while let Ok(event) = event_rx.try_recv() {
            if let Some(ref cb) = output_callback
                && let Some(line) = format_swarm_event_for_output(&event)
            {
                cb(line);
            }
        }

        final_result.ok_or_else(|| anyhow::anyhow!("Swarm execution returned no result"))?
    } else {
        let mut executor = SwarmExecutor::new(swarm_config);
        if let Some(bus) = bus {
            executor = executor.with_bus(Arc::clone(bus));
        }
        executor.execute(prompt, strategy).await?
    };

    let final_text = if swarm_result.result.trim().is_empty() {
        if swarm_result.success {
            "Swarm completed without textual output.".to_string()
        } else {
            "Swarm finished with failures and no textual output.".to_string()
        }
    } else {
        swarm_result.result.clone()
    };

    session.add_message(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: final_text.clone(),
        }],
    });
    session.save().await?;

    Ok((
        crate::session::SessionResult {
            text: final_text,
            session_id: session.id.clone(),
        },
        swarm_result.success,
    ))
}

/// Execute a session with the given auto-approve policy
/// Optionally streams output chunks via the callback
async fn execute_session_with_policy(
    session: &mut Session,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<crate::session::SessionResult> {
    use crate::provider::{ContentPart, Message, ProviderRegistry, Role, parse_model_string};
    use std::sync::Arc;

    let per_task_keys = session
        .metadata
        .provider_keys
        .as_ref()
        .and_then(|value| match crate::provider::PerTaskProviderKeys::from_value(value) {
            Ok(keys) => keys,
            Err(err) => {
                tracing::warn!(error = %err, "Invalid per-task provider key payload; falling back to platform registry");
                None
            }
        });

    // Load provider registry from claim-injected tenant keys, falling back to Vault/env platform keys.
    let registry = if let Some(ref keys) = per_task_keys {
        let registry = keys.build_registry();
        if registry.list().is_empty() {
            tracing::warn!(
                "Per-task provider key payload produced no providers; falling back to platform registry"
            );
            ProviderRegistry::from_vault().await.context(
                "Failed to load provider registry from Vault — check VAULT_ADDR/VAULT_TOKEN",
            )?
        } else {
            tracing::info!(
                source = keys.source(),
                providers = ?keys.provider_names(),
                "Using tenant-scoped per-task provider registry"
            );
            registry
        }
    } else {
        ProviderRegistry::from_vault()
            .await
            .context("Failed to load provider registry from Vault — check VAULT_ADDR/VAULT_TOKEN")?
    };
    let providers = registry.list();
    tracing::info!("Available providers: {:?}", providers);

    if providers.is_empty() {
        anyhow::bail!(
            "No LLM providers available (0 providers loaded). \
             Configure API keys in HashiCorp Vault or set environment variables. \
             Vault address: {}",
            std::env::var("VAULT_ADDR").unwrap_or_else(|_| "(not set)".into())
        );
    }

    // Parse model string
    let (provider_name, model_id) = if let Some(ref model_str) = session.metadata.model {
        let (prov, model) = parse_model_string(model_str);
        let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
        if prov.is_some() {
            (prov.map(|s| s.to_string()), model.to_string())
        } else if providers.contains(&model) {
            (Some(model.to_string()), String::new())
        } else {
            (None, model.to_string())
        }
    } else {
        (None, String::new())
    };

    let provider_slice = providers.as_slice();
    if let Some(explicit_provider) = provider_name.as_deref()
        && !providers.contains(&explicit_provider)
    {
        anyhow::bail!(
            "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
            explicit_provider,
            providers.join(", ")
        );
    }

    // Determine which provider to use, preferring explicit request first, then model tier.
    let selected_provider = provider_name
        .as_deref()
        .unwrap_or_else(|| choose_provider_for_tier(provider_slice, model_tier));

    let provider = registry
        .get(selected_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

    // Add user message
    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });

    // Generate title
    if session.title.is_none() {
        session.generate_title().await?;
    }

    // Determine model.
    let model = if !model_id.is_empty() {
        model_id
    } else {
        default_model_for_provider(selected_provider, model_tier)
    };

    // Create tool registry with filtering based on auto-approve policy
    let workspace_dir = session
        .metadata
        .directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let tool_registry = create_filtered_registry(
        Arc::clone(&provider),
        model.clone(),
        auto_approve,
        &workspace_dir,
        output_callback.clone(),
    );
    let tool_definitions = tool_registry.definitions();

    let temperature = if prefers_temperature_one(&model) {
        Some(1.0)
    } else {
        Some(0.7)
    };

    tracing::info!(
        "Using model: {} via provider: {} (tier: {:?})",
        model,
        selected_provider,
        model_tier
    );
    tracing::info!(
        "Available tools: {} (auto_approve: {:?})",
        tool_definitions.len(),
        auto_approve
    );

    // Build system prompt
    let system_prompt = crate::agent::builtin::build_system_prompt(&workspace_dir);

    let mut final_output = String::new();
    let max_steps = 50;

    for step in 1..=max_steps {
        tracing::info!(step = step, "Agent step starting");

        let response = crate::session::context::complete_with_context(
            Arc::clone(&provider),
            session,
            &model,
            &system_prompt,
            &tool_definitions,
            crate::session::context::RequestOptions {
                temperature,
                top_p: None,
                max_tokens: Some(8192),
                force_keep_last: None,
            },
        )
        .await?;

        crate::telemetry::TOKEN_USAGE.record_model_usage(
            &model,
            response.usage.prompt_tokens as u64,
            response.usage.completion_tokens as u64,
        );

        // Extract tool calls
        let tool_calls: Vec<(String, String, serde_json::Value)> = response
            .message
            .content
            .iter()
            .filter_map(|part| {
                if let ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } = part
                {
                    let args: serde_json::Value =
                        serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                    Some((id.clone(), name.clone(), args))
                } else {
                    None
                }
            })
            .collect();

        // Collect text output and stream it
        for part in &response.message.content {
            if let ContentPart::Text { text } = part
                && !text.is_empty()
            {
                final_output.push_str(text);
                final_output.push('\n');
                if let Some(ref cb) = output_callback {
                    cb(text.clone());
                }
            }
        }

        // If no tool calls, we're done
        if tool_calls.is_empty() {
            session.add_message(response.message.clone());
            break;
        }

        session.add_message(response.message.clone());

        tracing::info!(
            step = step,
            num_tools = tool_calls.len(),
            "Executing tool calls"
        );

        // Execute each tool call
        for (tool_id, tool_name, tool_input) in tool_calls {
            tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

            // Stream tool start event
            if let Some(ref cb) = output_callback {
                cb(format!("[tool:start:{}]", tool_name));
            }

            // Check if tool is allowed based on auto-approve policy
            if !is_tool_allowed(&tool_name, auto_approve) {
                let msg = format!(
                    "Tool '{}' requires approval but auto-approve policy is {:?}",
                    tool_name, auto_approve
                );
                tracing::warn!(tool = %tool_name, "Tool blocked by auto-approve policy");
                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content: msg,
                    }],
                });
                continue;
            }

            let content = if let Some(tool) = tool_registry.get(&tool_name) {
                let exec_result: Result<crate::tool::ToolResult> =
                    tool.execute(tool_input.clone()).await;
                match exec_result {
                    Ok(result) => {
                        tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                        if let Some(ref cb) = output_callback {
                            let status = if result.success { "ok" } else { "err" };
                            cb(format!(
                                "[tool:{}:{}] {}",
                                tool_name,
                                status,
                                crate::util::truncate_bytes_safe(&result.output, 500)
                            ));
                        }
                        result.output
                    }
                    Err(e) => {
                        tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                        if let Some(ref cb) = output_callback {
                            cb(format!("[tool:{}:err] {}", tool_name, e));
                        }
                        format!("Error: {}", e)
                    }
                }
            } else {
                tracing::warn!(tool = %tool_name, "Tool not found");
                format!("Error: Unknown tool '{}'", tool_name)
            };

            session.add_message(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: tool_id,
                    content,
                }],
            });
        }
    }

    session.save().await?;

    Ok(crate::session::SessionResult {
        text: final_output.trim().to_string(),
        session_id: session.id.clone(),
    })
}

/// Start the heartbeat background task
/// Returns a JoinHandle that can be used to cancel the heartbeat
pub fn start_heartbeat(
    client: Client,
    server: String,
    token: Option<String>,
    heartbeat_state: HeartbeatState,
    processing: Arc<Mutex<HashSet<String>>>,
    cognition_config: CognitionHeartbeatConfig,
    task_progress: Arc<Mutex<task_timeline::TaskProgressState>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut consecutive_failures = 0u32;
        const MAX_FAILURES: u32 = 3;
        const HEARTBEAT_INTERVAL_SECS: u64 = 30;
        const COGNITION_RETRY_COOLDOWN_SECS: u64 = 300;
        let mut cognition_payload_disabled_until: Option<Instant> = None;

        let mut interval =
            tokio::time::interval(tokio::time::Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            // Update task count from processing set
            let active_count = processing.lock().await.len();
            heartbeat_state.set_task_count(active_count).await;

            // Determine status based on active tasks
            let status = if active_count > 0 {
                WorkerStatus::Processing
            } else {
                WorkerStatus::Idle
            };
            heartbeat_state.set_status(status).await;

            // Send heartbeat
            let url = format!(
                "{}/v1/agent/workers/{}/heartbeat",
                server, heartbeat_state.worker_id
            );
            let mut req = client.post(&url);

            if let Some(ref t) = token {
                req = req.bearer_auth(t);
            }

            let status_str = heartbeat_state.status.lock().await.as_str().to_string();
            let sub_agents = heartbeat_state.sub_agents_snapshot().await;

            // Include task pipeline progress in heartbeat
            let progress_snapshot = task_progress.lock().await.clone();
            let task_progress_payload = if progress_snapshot.task_id.is_some() {
                Some(serde_json::json!({
                    "task_id": progress_snapshot.task_id,
                    "current_checkpoint": progress_snapshot.current_checkpoint,
                    "elapsed_secs": format!("{:.1}", progress_snapshot.elapsed_secs),
                    "remaining_secs": format!("{:.1}", progress_snapshot.remaining_secs),
                    "budget_pct_used": format!("{:.1}%", progress_snapshot.budget_pct_used),
                    "checkpoints_reached": progress_snapshot.checkpoints_reached.len(),
                    "last_detail": progress_snapshot.last_detail,
                }))
            } else {
                None
            };

            let base_payload = serde_json::json!({
                "worker_id": &heartbeat_state.worker_id,
                "agent_name": &heartbeat_state.agent_name,
                "status": status_str,
                "active_task_count": active_count,
                "sub_agents": sub_agents,
            });
            let mut payload = base_payload.clone();
            let mut included_cognition_payload = false;
            let cognition_payload_allowed = cognition_payload_disabled_until
                .map(|until| Instant::now() >= until)
                .unwrap_or(true);

            if cognition_config.enabled
                && cognition_payload_allowed
                && let Some(cognition_payload) =
                    fetch_cognition_heartbeat_payload(&client, &cognition_config).await
                && let Some(obj) = payload.as_object_mut()
            {
                obj.insert("cognition".to_string(), cognition_payload);
                included_cognition_payload = true;
            }

            // Include task pipeline progress if a task is active
            if let Some(progress_json) = task_progress_payload {
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("task_progress".to_string(), progress_json);
                }
            }

            match req.json(&payload).send().await {
                Ok(res) => {
                    if res.status().is_success() {
                        consecutive_failures = 0;
                        tracing::debug!(
                            worker_id = %heartbeat_state.worker_id,
                            status = status_str,
                            active_tasks = active_count,
                            "Heartbeat sent successfully"
                        );
                    } else if included_cognition_payload && res.status().is_client_error() {
                        tracing::warn!(
                            worker_id = %heartbeat_state.worker_id,
                            status = %res.status(),
                            "Heartbeat cognition payload rejected, retrying without cognition payload"
                        );

                        let mut retry_req = client.post(&url);
                        if let Some(ref t) = token {
                            retry_req = retry_req.bearer_auth(t);
                        }

                        match retry_req.json(&base_payload).send().await {
                            Ok(retry_res) if retry_res.status().is_success() => {
                                cognition_payload_disabled_until = Some(
                                    Instant::now()
                                        + Duration::from_secs(COGNITION_RETRY_COOLDOWN_SECS),
                                );
                                consecutive_failures = 0;
                                tracing::warn!(
                                    worker_id = %heartbeat_state.worker_id,
                                    retry_after_secs = COGNITION_RETRY_COOLDOWN_SECS,
                                    "Paused cognition heartbeat payload after schema rejection"
                                );
                            }
                            Ok(retry_res) => {
                                consecutive_failures += 1;
                                tracing::warn!(
                                    worker_id = %heartbeat_state.worker_id,
                                    status = %retry_res.status(),
                                    failures = consecutive_failures,
                                    "Heartbeat failed even after retry without cognition payload"
                                );
                            }
                            Err(e) => {
                                consecutive_failures += 1;
                                tracing::warn!(
                                    worker_id = %heartbeat_state.worker_id,
                                    error = %e,
                                    failures = consecutive_failures,
                                    "Heartbeat retry without cognition payload failed"
                                );
                            }
                        }
                    } else {
                        consecutive_failures += 1;
                        tracing::warn!(
                            worker_id = %heartbeat_state.worker_id,
                            status = %res.status(),
                            failures = consecutive_failures,
                            "Heartbeat failed"
                        );
                    }
                }
                Err(e) => {
                    consecutive_failures += 1;
                    tracing::warn!(
                        worker_id = %heartbeat_state.worker_id,
                        error = %e,
                        failures = consecutive_failures,
                        "Heartbeat request failed"
                    );
                }
            }

            // Log error after 3 consecutive failures but do not terminate
            if consecutive_failures >= MAX_FAILURES {
                tracing::error!(
                    worker_id = %heartbeat_state.worker_id,
                    failures = consecutive_failures,
                    "Heartbeat failed {} consecutive times - worker will continue running and attempt reconnection via SSE loop",
                    MAX_FAILURES
                );
                // Reset counter to avoid spamming error logs
                consecutive_failures = 0;
            }
        }
    })
}

async fn fetch_cognition_heartbeat_payload(
    client: &Client,
    config: &CognitionHeartbeatConfig,
) -> Option<serde_json::Value> {
    let status_url = format!("{}/v1/cognition/status", config.source_base_url);
    let status_res = tokio::time::timeout(
        Duration::from_millis(config.request_timeout_ms),
        client.get(status_url).send(),
    )
    .await
    .ok()?
    .ok()?;

    if !status_res.status().is_success() {
        return None;
    }

    let status: CognitionStatusSnapshot = status_res.json().await.ok()?;
    let mut payload = serde_json::json!({
        "running": status.running,
        "last_tick_at": status.last_tick_at,
        "active_persona_count": status.active_persona_count,
        "events_buffered": status.events_buffered,
        "snapshots_buffered": status.snapshots_buffered,
        "loop_interval_ms": status.loop_interval_ms,
    });

    if config.include_thought_summary {
        let snapshot_url = format!("{}/v1/cognition/snapshots/latest", config.source_base_url);
        let snapshot_res = tokio::time::timeout(
            Duration::from_millis(config.request_timeout_ms),
            client.get(snapshot_url).send(),
        )
        .await
        .ok()
        .and_then(Result::ok);

        if let Some(snapshot_res) = snapshot_res
            && snapshot_res.status().is_success()
            && let Ok(snapshot) = snapshot_res.json::<CognitionLatestSnapshot>().await
            && let Some(obj) = payload.as_object_mut()
        {
            obj.insert(
                "latest_snapshot_at".to_string(),
                serde_json::Value::String(snapshot.generated_at),
            );
            obj.insert(
                "latest_thought".to_string(),
                serde_json::Value::String(trim_for_heartbeat(
                    &snapshot.summary,
                    config.summary_max_chars,
                )),
            );
            if let Some(model) = snapshot
                .metadata
                .get("model")
                .and_then(serde_json::Value::as_str)
            {
                obj.insert(
                    "latest_thought_model".to_string(),
                    serde_json::Value::String(model.to_string()),
                );
            }
            if let Some(source) = snapshot
                .metadata
                .get("source")
                .and_then(serde_json::Value::as_str)
            {
                obj.insert(
                    "latest_thought_source".to_string(),
                    serde_json::Value::String(source.to_string()),
                );
            }
        }
    }

    Some(payload)
}

fn trim_for_heartbeat(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.trim().to_string();
    }

    let mut trimmed = String::with_capacity(max_chars + 3);
    for ch in input.chars().take(max_chars) {
        trimmed.push(ch);
    }
    trimmed.push_str("...");
    trimmed.trim().to_string()
}

fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .and_then(|v| match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default)
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}

fn maybe_configure_repo_git_auth_from_entry(entry: &serde_json::Value) {
    let Some(workspace_id) = entry.get("id").and_then(|v| v.as_str()) else {
        return;
    };
    let Some(path) = entry.get("path").and_then(|v| v.as_str()) else {
        return;
    };
    let has_git_remote = entry
        .get("git_url")
        .and_then(|v| v.as_str())
        .is_some_and(|value| !value.trim().is_empty());
    if !has_git_remote {
        return;
    }

    let repo_path = Path::new(path);
    if !repo_path.join(".git").exists() {
        return;
    }

    if let Err(error) = configure_repo_git_auth(repo_path, workspace_id) {
        tracing::debug!(
            workspace_id,
            path,
            error = %error,
            "Workspace sync could not install Git credential helper"
        );
    }
    configure_repo_git_github_app_from_agent_config(repo_path, entry.get("agent_config"));
}

fn configure_repo_git_github_app_from_agent_config(
    repo_path: &Path,
    agent_config: Option<&serde_json::Value>,
) {
    let installation_id = agent_config
        .and_then(|value| value.get("git_auth"))
        .and_then(|value| value.get("github_app"))
        .and_then(|value| value.get("installation_id"))
        .and_then(|value| value.as_str());
    let app_id = agent_config
        .and_then(|value| value.get("git_auth"))
        .and_then(|value| value.get("github_app"))
        .and_then(|value| value.get("app_id"))
        .and_then(|value| value.as_str());
    if let Err(error) = configure_repo_git_github_app(repo_path, installation_id, app_id) {
        tracing::debug!(path = %repo_path.display(), error = %error, "Failed to persist GitHub App repo metadata");
    }
}

/// Start a background task that periodically fetches the server's workspace list
/// and auto-registers any whose path exists on this machine's filesystem.
/// This lets workers pick up new codebases without restarting.
fn start_workspace_sync(
    client: Client,
    server: String,
    token: Option<String>,
    shared_codebases: Arc<Mutex<Vec<String>>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        const POLL_INTERVAL_SECS: u64 = 60;
        let mut interval = tokio::time::interval(Duration::from_secs(POLL_INTERVAL_SECS));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        interval.tick().await; // skip the immediate first tick -- let the worker settle first

        loop {
            interval.tick().await;
            if let Err(e) =
                sync_workspaces_from_server(&client, &server, &token, &shared_codebases).await
            {
                tracing::warn!("Workspace sync failed: {}", e);
            }
        }
    })
}

/// Fetch the server's workspace/codebase list and add any locally-present paths
/// that are not already in the worker's registered codebases.
/// New paths take effect on the next SSE reconnect (re-register re-sends X-Workspaces).
async fn sync_workspaces_from_server(
    client: &Client,
    server: &str,
    token: &Option<String>,
    shared_codebases: &Arc<Mutex<Vec<String>>>,
) -> Result<()> {
    let mut req = client.get(format!("{}/v1/agent/workspaces", server));
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        tracing::debug!(
            status = %res.status(),
            "Workspace sync: server returned non-success, skipping"
        );
        return Ok(());
    }

    let data: serde_json::Value = res.json().await?;

    // Server returns { workspaces: [...] } or { codebases: [...] }
    let entries = if let Some(arr) = data.as_array() {
        arr.clone()
    } else {
        data["workspaces"]
            .as_array()
            .or_else(|| data["codebases"].as_array())
            .cloned()
            .unwrap_or_default()
    };

    let mut new_paths: Vec<String> = Vec::new();
    {
        let current = shared_codebases.lock().await;
        for entry in &entries {
            maybe_configure_repo_git_auth_from_entry(entry);
            let path = match entry["path"].as_str().filter(|p| !p.is_empty()) {
                Some(p) => p,
                None => continue,
            };
            // Only auto-register if the path physically exists on this machine
            // and is not already in the codebases list
            if std::path::Path::new(path).exists() && !current.iter().any(|c| c.as_str() == path) {
                new_paths.push(path.to_string());
            }
        }
    }

    if !new_paths.is_empty() {
        let mut current = shared_codebases.lock().await;
        for path in &new_paths {
            tracing::info!(
                path = %path,
                "Workspace sync: auto-discovered local path, adding to codebases"
            );
            current.push(path.clone());
        }
        tracing::info!(
            added = new_paths.len(),
            total = current.len(),
            "Workspace sync complete -- new paths take effect on next reconnect"
        );
    } else {
        tracing::debug!("Workspace sync: no new local paths found");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn metadata_lookup_reads_nested_forage_keys() {
        let metadata = json!({
            "forage": {
                "execute": true,
                "top": 5,
            }
        })
        .as_object()
        .cloned()
        .unwrap();

        assert_eq!(metadata_bool(&metadata, &["execute"]), Some(true));
        assert_eq!(metadata_usize(&metadata, &["top"]), Some(5));
    }

    #[test]
    fn build_forage_args_defaults_prompt_to_moonshot_and_local_mode() {
        let metadata = serde_json::Map::new();
        let args = build_forage_args(
            "Expand enterprise adoption in regulated markets",
            "forage task",
            &metadata,
            Some("openrouter/z-ai/glm-5".to_string()),
        );

        assert_eq!(
            args.moonshots,
            vec!["Expand enterprise adoption in regulated markets".to_string()]
        );
        assert!(args.no_s3);
        assert_eq!(args.model.as_deref(), Some("openrouter/z-ai/glm-5"));
        assert_eq!(args.execution_engine, "run");
    }

    #[test]
    fn build_forage_args_honors_nested_forage_configuration() {
        let metadata = json!({
            "forage": {
                "top": 7,
                "loop": true,
                "max_cycles": 2,
                "execute": true,
                "no_s3": false,
                "moonshots": ["Ship autonomous OKR execution", "Reduce operator toil"],
                "moonshot_required": true,
                "moonshot_min_alignment": "0.25",
                "execution_engine": "swarm",
                "run_timeout_secs": 1200,
                "fail_fast": true,
                "swarm_strategy": "stage",
                "swarm_max_subagents": 4,
                "swarm_max_steps": 42,
                "swarm_subagent_timeout_secs": 180
            }
        })
        .as_object()
        .cloned()
        .unwrap();

        let args = build_forage_args("fallback prompt", "title", &metadata, None);

        assert_eq!(args.top, 7);
        assert!(args.loop_mode);
        assert_eq!(args.max_cycles, 2);
        assert!(args.execute);
        assert!(!args.no_s3);
        assert_eq!(args.moonshots.len(), 2);
        assert!(args.moonshot_required);
        assert!((args.moonshot_min_alignment - 0.25).abs() < f64::EPSILON);
        assert_eq!(args.execution_engine, "swarm");
        assert_eq!(args.run_timeout_secs, 1200);
        assert!(args.fail_fast);
        assert_eq!(args.swarm_strategy, "stage");
        assert_eq!(args.swarm_max_subagents, 4);
        assert_eq!(args.swarm_max_steps, 42);
        assert_eq!(args.swarm_subagent_timeout_secs, 180);
    }

    #[test]
    fn resolve_worker_id_prefers_env() {
        let original = std::env::var("CODETETHER_WORKER_ID").ok();
        unsafe {
            std::env::set_var("CODETETHER_WORKER_ID", "harvester-test-worker");
        }
        let resolved = resolve_worker_id();
        match original {
            Some(value) => unsafe {
                std::env::set_var("CODETETHER_WORKER_ID", value);
            },
            None => unsafe {
                std::env::remove_var("CODETETHER_WORKER_ID");
            },
        }
        assert_eq!(resolved, "harvester-test-worker");
    }

    #[test]
    fn advertised_interfaces_include_http_and_bus_urls() {
        let interfaces = advertised_interfaces(Some("http://worker.test:8080/"));
        assert_eq!(interfaces["http"]["base_url"], "http://worker.test:8080");
        assert_eq!(
            interfaces["bus"]["stream_url"],
            "http://worker.test:8080/v1/bus/stream"
        );
        assert_eq!(
            interfaces["bus"]["publish_url"],
            "http://worker.test:8080/v1/bus/publish"
        );
    }

    #[test]
    fn advertised_interfaces_omit_empty_public_url() {
        assert_eq!(advertised_interfaces(None), serde_json::json!({}));
        assert_eq!(advertised_interfaces(Some("   ")), serde_json::json!({}));
    }

    #[tokio::test]
    async fn reserve_task_slot_enforces_capacity() {
        let processing = Arc::new(Mutex::new(HashSet::new()));

        assert_eq!(
            reserve_task_slot(&processing, "task-1", 2).await,
            TaskReservation::Reserved
        );
        assert_eq!(
            reserve_task_slot(&processing, "task-1", 2).await,
            TaskReservation::AlreadyProcessing
        );
        assert_eq!(
            reserve_task_slot(&processing, "task-2", 2).await,
            TaskReservation::Reserved
        );
        assert_eq!(
            reserve_task_slot(&processing, "task-3", 2).await,
            TaskReservation::AtCapacity
        );
    }

    #[test]
    fn normalize_max_concurrent_tasks_never_returns_zero() {
        assert_eq!(normalize_max_concurrent_tasks(0), 1);
        assert_eq!(normalize_max_concurrent_tasks(3), 3);
    }
}