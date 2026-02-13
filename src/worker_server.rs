//! Worker HTTP Server
//!
//! Minimal HTTP server for the A2A worker that provides:
//! - /health - liveness probe
//! - /ready - readiness probe
//! - /task - POST endpoint for CloudEvents (Knative integration)
//!
//! This enables Kubernetes probes and ingress routing to work.

use crate::a2a::worker::HeartbeatState;
use crate::cli::WorkerServerArgs;
use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Worker server state shared across handlers
#[derive(Clone)]
pub struct WorkerServerState {
    /// Heartbeat state from the worker (used to determine readiness)
    pub heartbeat_state: Option<Arc<HeartbeatState>>,
    /// Whether the SSE stream is currently connected
    pub connected: Arc<Mutex<bool>>,
    /// Worker ID for identification
    pub worker_id: Arc<Mutex<Option<String>>>,
}

impl WorkerServerState {
    pub fn new() -> Self {
        Self {
            heartbeat_state: None,
            connected: Arc::new(Mutex::new(false)),
            worker_id: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_heartbeat(mut self, state: Arc<HeartbeatState>) -> Self {
        self.heartbeat_state = Some(state);
        self
    }

    pub async fn set_connected(&self, connected: bool) {
        *self.connected.lock().await = connected;
    }

    pub async fn set_worker_id(&self, worker_id: String) {
        *self.worker_id.lock().await = Some(worker_id);
    }

    pub async fn is_ready(&self) -> bool {
        let connected = *self.connected.lock().await;
        // Ready if we have a connection to the A2A server
        // Optional: could also check heartbeat_state for active task count
        connected
    }
}

/// Start the worker HTTP server
pub async fn start_worker_server(args: WorkerServerArgs) -> Result<()> {
    let addr = format!("{}:{}", args.hostname, args.port);
    
    tracing::info!("Starting worker HTTP server on http://{}", addr);
    
    let state = WorkerServerState::new();
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/task", post(receive_task))
        .route("/worker/status", get(worker_status))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Worker HTTP server listening on http://{}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Health check - always returns OK if the server is running
async fn health() -> &'static str {
    "ok"
}

/// Readiness check - returns OK only when connected to A2A server
async fn ready(State(state): State<WorkerServerState>) -> (StatusCode, String) {
    if state.is_ready().await {
        (StatusCode::OK, "ready".to_string())
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "not connected".to_string())
    }
}

/// Worker status endpoint - returns detailed worker state
async fn worker_status(State(state): State<WorkerServerState>) -> Json<WorkerStatusResponse> {
    let connected = *state.connected.lock().await;
    let worker_id = state.worker_id.lock().await.clone();
    
    let heartbeat_info = if let Some(ref hb_state) = state.heartbeat_state {
        let status = hb_state.status.lock().await;
        let task_count = hb_state.active_task_count.lock().await;
        Some(HeartbeatInfo {
            status: status.as_str().to_string(),
            active_tasks: *task_count,
            agent_name: hb_state.agent_name.clone(),
        })
    } else {
        None
    };
    
    Json(WorkerStatusResponse {
        connected,
        worker_id,
        heartbeat: heartbeat_info,
    })
}

/// Receive CloudEvents POST (for Knative integration)
/// This endpoint receives tasks pushed via Knative Eventing
async fn receive_task(Json(payload): Json<serde_json::Value>) -> (StatusCode, String) {
    // Log the received task
    let task_id = payload.get("task_id")
        .or_else(|| payload.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    tracing::info!("Received task via CloudEvent: {}", task_id);
    
    // For now, just acknowledge receipt. The worker will pick up tasks
    // via SSE polling. This endpoint enables Knative Push delivery.
    // TODO: Queue task for processing if not already handled
    
    (StatusCode::ACCEPTED, format!("task {} received", task_id))
}

/// Response types
#[derive(Serialize)]
struct WorkerStatusResponse {
    connected: bool,
    worker_id: Option<String>,
    heartbeat: Option<HeartbeatInfo>,
}

#[derive(Serialize)]
struct HeartbeatInfo {
    status: String,
    active_tasks: usize,
    agent_name: String,
}
