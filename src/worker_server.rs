//! Worker HTTP Server
//!
//! Minimal HTTP server for the A2A worker that provides:
//! - /health - liveness probe
//! - /ready - readiness probe
//! - /task - POST endpoint for CloudEvents (Knative integration)
//!
//! This enables Kubernetes probes and ingress routing to work.

use crate::a2a::worker::{HeartbeatState, WorkerStatus};
use crate::bus::{AgentBus, BusEnvelope};
use crate::cli::WorkerServerArgs;
use anyhow::Result;
use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use futures::stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast, mpsc};

/// Worker server state shared across handlers
#[derive(Clone)]
pub struct WorkerServerState {
    /// Heartbeat state from the worker (used to determine readiness)
    pub heartbeat_state: Option<Arc<HeartbeatState>>,
    /// Whether the SSE stream is currently connected
    pub connected: Arc<Mutex<bool>>,
    /// Worker ID for identification
    pub worker_id: Arc<Mutex<Option<String>>>,
    /// The actual heartbeat state (internal, for sharing with HTTP server)
    internal_heartbeat: Arc<Mutex<Option<Arc<HeartbeatState>>>>,
    /// Channel to notify worker of new tasks (from CloudEvents)
    task_notification_tx: Arc<Mutex<Option<mpsc::Sender<String>>>>,
    /// Agent bus for inter-agent messaging (exposed via /v1/bus/*)
    bus: Arc<Mutex<Option<Arc<AgentBus>>>>,
}

impl Default for WorkerServerState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkerServerState {
    pub fn new() -> Self {
        Self {
            heartbeat_state: None,
            connected: Arc::new(Mutex::new(false)),
            worker_id: Arc::new(Mutex::new(None)),
            internal_heartbeat: Arc::new(Mutex::new(None)),
            task_notification_tx: Arc::new(Mutex::new(None)),
            bus: Arc::new(Mutex::new(None)),
        }
    }

    /// Attach the agent bus (can be called after construction from the worker task)
    pub async fn set_bus(&self, bus: Arc<AgentBus>) {
        *self.bus.lock().await = Some(bus);
    }

    /// Set the task notification channel (called from worker)
    pub async fn set_task_notification_channel(&self, tx: mpsc::Sender<String>) {
        *self.task_notification_tx.lock().await = Some(tx);
    }

    /// Notify worker of a new task (called from /task endpoint)
    pub async fn notify_new_task(&self, task_id: &str) {
        if let Some(ref tx) = *self.task_notification_tx.lock().await {
            let _ = tx.send(task_id.to_string()).await;
            tracing::debug!("Notified worker of new task: {}", task_id);
        }
    }

    #[allow(dead_code)]
    pub fn with_heartbeat(mut self, state: Option<Arc<HeartbeatState>>) -> Self {
        self.heartbeat_state = state.clone();
        self
    }

    /// Set the heartbeat state (called from worker)
    pub async fn set_heartbeat_state(&self, state: Arc<HeartbeatState>) {
        *self.internal_heartbeat.lock().await = Some(state);
    }

    /// Get the heartbeat state (for HTTP server)
    #[allow(dead_code)]
    pub async fn heartbeat_state(&self) -> Arc<HeartbeatState> {
        let guard: Option<Arc<HeartbeatState>> = self.internal_heartbeat.lock().await.clone();
        guard.unwrap_or_else(|| {
            // Create a default heartbeat state if not set
            let state = HeartbeatState::new("unknown".to_string(), "unknown".to_string());
            Arc::new(state)
        })
    }

    pub async fn set_connected(&self, connected: bool) {
        *self.connected.lock().await = connected;
    }

    pub async fn set_worker_id(&self, worker_id: String) {
        *self.worker_id.lock().await = Some(worker_id);
    }

    #[allow(dead_code)]
    pub async fn worker_id(&self) -> String {
        self.worker_id.lock().await.clone().unwrap_or_default()
    }

    #[allow(dead_code)]
    pub async fn is_connected(&self) -> bool {
        *self.connected.lock().await
    }

    pub async fn is_ready(&self) -> bool {
        // Ready if we have a connection to the A2A server
        // Optional: could also check heartbeat_state for active task count
        *self.connected.lock().await
    }
}

/// Start the worker HTTP server with custom state
pub async fn start_worker_server_with_state(
    args: WorkerServerArgs,
    state: WorkerServerState,
) -> Result<()> {
    let addr = format!("{}:{}", args.hostname, args.port);

    tracing::info!("Starting worker HTTP server on http://{}", addr);

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/task", post(receive_task))
        .route("/worker/status", get(worker_status))
        .route("/v1/bus/stream", get(stream_bus_events))
        .route("/v1/bus/publish", post(publish_bus_event))
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
    let heartbeat_state = state
        .internal_heartbeat
        .lock()
        .await
        .clone()
        .or_else(|| state.heartbeat_state.clone());

    let heartbeat_info = if let Some(ref hb_state) = heartbeat_state {
        let status: WorkerStatus = *hb_state.status.lock().await;
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
async fn receive_task(
    State(state): State<WorkerServerState>,
    Json(payload): Json<serde_json::Value>,
) -> StatusCode {
    // Extract task_id from CloudEvent payload
    let task_id = payload
        .get("task_id")
        .or_else(|| payload.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    tracing::info!("Received task via CloudEvent: {}", task_id);

    // Notify the worker loop to pick up this task
    state.notify_new_task(task_id).await;

    // CloudEvent subscribers should return an empty 2xx response body.
    // Non-empty non-CloudEvent payloads trigger retries in Knative Broker Filter.
    StatusCode::ACCEPTED
}

/// SSE stream of agent bus events — subscribe to live bus messages.
/// Streams all topics; use the `topic` query param to filter (e.g. `?topic=task.*`).
async fn stream_bus_events(State(state): State<WorkerServerState>, req: Request<Body>) -> Response {
    let bus = state.bus.lock().await.clone();
    let Some(bus) = bus else {
        // Bus not attached — return an empty keep-alive stream
        let empty = stream::empty::<Result<Event, Infallible>>();
        return Sse::new(empty)
            .keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
            .into_response();
    };

    let topic_filter: Option<String> = req.uri().query().and_then(|q| {
        q.split('&')
            .filter_map(|pair| pair.split_once('='))
            .find(|(k, _)| *k == "topic")
            .map(|(_, v)| v.to_owned())
    });

    let bus_handle = bus.handle("worker_server_bus_stream");
    let rx: broadcast::Receiver<BusEnvelope> = bus_handle.into_receiver();

    let event_stream = stream::unfold(rx, move |mut rx| {
        let filter = topic_filter.clone();
        async move {
            match rx.recv().await {
                Ok(envelope) => {
                    let allowed = filter
                        .as_deref()
                        .map(|pat| bus_topic_matches(&envelope.topic, pat))
                        .unwrap_or(true);

                    if allowed {
                        let payload =
                            serde_json::to_string(&envelope).unwrap_or_else(|_| "{}".to_string());
                        Some((
                            Ok::<Event, Infallible>(Event::default().event("bus").data(payload)),
                            rx,
                        ))
                    } else {
                        Some((
                            Ok::<Event, Infallible>(Event::default().event("keepalive").data("")),
                            rx,
                        ))
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => Some((
                    Ok(Event::default().event("lag").data(format!("skipped {}", n))),
                    rx,
                )),
                Err(broadcast::error::RecvError::Closed) => None,
            }
        }
    });

    Sse::new(event_stream)
        .keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
        .into_response()
}

/// Wildcard topic matching (supports `*` and `prefix.*` patterns).
fn bus_topic_matches(topic: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return topic.starts_with(prefix);
    }
    topic == pattern
}

/// Publish a message to the agent bus.
#[derive(Deserialize)]
struct BusPublishRequest {
    topic: String,
    payload: serde_json::Value,
}

async fn publish_bus_event(
    State(state): State<WorkerServerState>,
    Json(req): Json<BusPublishRequest>,
) -> StatusCode {
    let bus = state.bus.lock().await.clone();
    let Some(bus) = bus else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };
    let handle = bus.handle("worker_server_publish");
    handle.send(
        &req.topic,
        crate::bus::BusMessage::SharedResult {
            key: req.topic.clone(),
            value: req.payload,
            tags: vec![],
        },
    );
    StatusCode::ACCEPTED
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
