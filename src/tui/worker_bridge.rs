//! TUI Worker Bridge - connects the TUI to the A2A server substrate.
//!
//! This module enables the TUI to:
//! - Register itself as a worker with the A2A server
//! - Send heartbeats to maintain registration
//! - Receive incoming tasks via SSE stream
//! - Register/deregister sub-agents (relay, autochat, spawned agents)
//! - Forward bus events to the server for observability

use crate::a2a::worker::{
    CognitionHeartbeatConfig, HeartbeatState, WorkerStatus, register_worker, start_heartbeat,
};
use crate::bus::{AgentBus, BusEnvelope, BusMessage};
use crate::cli::auth::load_saved_credentials;
use crate::config::Config;
use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

/// Command sent to the worker bridge to register/deregister sub-agents
#[derive(Debug, Clone)]
pub enum WorkerBridgeCmd {
    /// Register a sub-agent with the A2A server
    RegisterAgent { name: String, instructions: String },
    /// Deregister a sub-agent
    DeregisterAgent { name: String },
    /// Update the processing status (for heartbeat)
    SetProcessing(bool),
}

/// Incoming task from the A2A server via SSE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingTask {
    pub task_id: String,
    pub message: String,
    pub from_agent: Option<String>,
}

/// Result of worker bridge initialization
pub struct TuiWorkerBridge {
    /// Worker ID assigned by the server
    pub worker_id: String,
    /// Worker name
    pub worker_name: String,
    /// Sender for commands (register/deregister agents)
    pub cmd_tx: mpsc::Sender<WorkerBridgeCmd>,
    /// Receiver for incoming tasks
    pub task_rx: mpsc::Receiver<IncomingTask>,
    /// Handle for the bridge task (for shutdown)
    pub handle: tokio::task::JoinHandle<()>,
}

impl TuiWorkerBridge {
    /// Spawn the worker bridge if server credentials are available.
    /// Returns None if no server is configured.
    pub async fn spawn(
        server_url: Option<String>,
        token: Option<String>,
        worker_name: Option<String>,
        bus: Arc<AgentBus>,
    ) -> Result<Option<Self>> {
        // Try to get server URL from config if not provided
        let server = match server_url {
            Some(url) => url,
            None => {
                let config = Config::load().await?;
                match config.a2a.server_url {
                    Some(url) => url,
                    None => {
                        tracing::debug!("No A2A server configured, worker bridge disabled");
                        return Ok(None);
                    }
                }
            }
        };

        // Try to get token from saved credentials if not provided
        let token = match token {
            Some(t) => Some(t),
            None => load_saved_credentials().map(|c| c.access_token),
        };

        // If no token, we can't register - but we could run in "read-only" mode
        // For now, let's require a token for registration
        let token = match token {
            Some(t) => t,
            None => {
                tracing::debug!("No A2A token available, worker bridge disabled");
                return Ok(None);
            }
        };

        // Generate worker ID and name
        let worker_id = crate::a2a::worker::generate_worker_id();
        let worker_name = worker_name.unwrap_or_else(|| format!("tui-{}", std::process::id()));

        tracing::info!(
            worker_id = %worker_id,
            worker_name = %worker_name,
            server = %server,
            "Starting TUI worker bridge"
        );

        // Create channels
        let (cmd_tx, cmd_rx) = mpsc::channel::<WorkerBridgeCmd>(32);
        let (task_tx, task_rx) = mpsc::channel::<IncomingTask>(64);

        // Create shared state
        let client = Client::new();
        let processing = Arc::new(Mutex::new(HashSet::<String>::new()));
        let heartbeat_state = HeartbeatState::new(worker_id.clone(), worker_name.clone());
        let cognition_config = CognitionHeartbeatConfig::from_env();
        let codebases = vec![
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| ".".to_string()),
        ];

        // Register with the server
        let token_opt = Some(token.clone());
        if let Err(e) = register_worker(
            &client,
            &server,
            &token_opt,
            &worker_id,
            &worker_name,
            &codebases,
        )
        .await
        {
            tracing::warn!("Failed to register worker with A2A server: {}", e);
            // Continue anyway - we can still try to process tasks
        }

        // Start heartbeat
        let heartbeat_handle = start_heartbeat(
            client.clone(),
            server.clone(),
            token_opt.clone(),
            heartbeat_state.clone(),
            processing.clone(),
            cognition_config,
        );

        // Spawn the main bridge task
        let handle = tokio::spawn({
            let server = server.clone();
            let token = token.clone();
            let worker_id = worker_id.clone();
            let worker_name = worker_name.clone();
            async move {
                // Background task: connect to SSE stream for incoming tasks
                let sse_handle = tokio::spawn({
                    let server = server.clone();
                    let token = token.clone();
                    let worker_id = worker_id.clone();
                    let worker_name = worker_name.clone();
                    async move {
                        loop {
                            let url = format!(
                                "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
                                server,
                                urlencoding::encode(&worker_name),
                                urlencoding::encode(&worker_id)
                            );

                            let req = Client::new()
                                .get(&url)
                                .header("Accept", "text/event-stream")
                                .header("X-Worker-ID", &worker_id)
                                .header("X-Agent-Name", &worker_name)
                                .bearer_auth(&token);

                            match req.send().await {
                                Ok(res) if res.status().is_success() => {
                                    tracing::info!("Connected to A2A task stream");
                                    let mut stream = res.bytes_stream();
                                    let mut buffer = String::new();

                                    while let Some(chunk) = stream.next().await {
                                        match chunk {
                                            Ok(bytes) => {
                                                buffer.push_str(&String::from_utf8_lossy(&bytes));

                                                // Process SSE events
                                                while let Some(pos) = buffer.find("\n\n") {
                                                    let event_str = buffer[..pos].to_string();
                                                    buffer = buffer[pos + 2..].to_string();

                                                    if let Some(data_line) = event_str
                                                        .lines()
                                                        .find(|l| l.starts_with("data:"))
                                                    {
                                                        let data = data_line
                                                            .trim_start_matches("data:")
                                                            .trim();
                                                        if data.is_empty() || data == "[DONE]" {
                                                            continue;
                                                        }

                                                        // Try to parse as task
                                                        if let Ok(task) = serde_json::from_str::<
                                                            serde_json::Value,
                                                        >(
                                                            data
                                                        ) {
                                                            let task_id = task
                                                                .get("task_id")
                                                                .or_else(|| task.get("id"))
                                                                .and_then(|v| v.as_str())
                                                                .unwrap_or("unknown")
                                                                .to_string();

                                                            let message = task
                                                                .get("message")
                                                                .or_else(|| task.get("text"))
                                                                .and_then(|v| v.as_str())
                                                                .unwrap_or("")
                                                                .to_string();

                                                            let from_agent = task
                                                                .get("from_agent")
                                                                .or_else(|| task.get("agent"))
                                                                .and_then(|v| v.as_str())
                                                                .map(String::from);

                                                            let incoming = IncomingTask {
                                                                task_id,
                                                                message,
                                                                from_agent,
                                                            };

                                                            if task_tx.send(incoming).await.is_err()
                                                            {
                                                                tracing::warn!(
                                                                    "Task receiver dropped, stopping SSE stream"
                                                                );
                                                                return;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                tracing::warn!("SSE stream error: {}", e);
                                                break;
                                            }
                                        }
                                    }
                                }
                                Ok(res) => {
                                    tracing::warn!(
                                        "Failed to connect to task stream: {}",
                                        res.status()
                                    );
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to connect to task stream: {}", e);
                                }
                            }

                            // Reconnect after delay
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                });

                // Handle commands and bus events
                let mut cmd_rx = cmd_rx;
                let mut bus_handle = bus.handle(&worker_id);

                loop {
                    tokio::select! {
                        // Handle command to register/deregister agents
                        Some(cmd) = cmd_rx.recv() => {
                            match cmd {
                                WorkerBridgeCmd::RegisterAgent { name, instructions: _ } => {
                                    tracing::info!(agent = %name, "Registering sub-agent with A2A server");
                                    // For now, just log - full implementation would POST to server
                                    // The server tracks sub-agents via the heartbeat's agent list
                                }
                                WorkerBridgeCmd::DeregisterAgent { name } => {
                                    tracing::info!(agent = %name, "Deregistering sub-agent from A2A server");
                                }
                                WorkerBridgeCmd::SetProcessing(processing) => {
                                    let status = if processing {
                                        WorkerStatus::Processing
                                    } else {
                                        WorkerStatus::Idle
                                    };
                                    heartbeat_state.set_status(status).await;
                                }
                            }
                        }
                        // Handle bus events - forward to server
                        Some(envelope) = bus_handle.recv() => {
                            // Forward interesting events to server for observability
                            if let Err(e) = forward_bus_event(&client, &server, &token, &worker_id, &envelope).await {
                                tracing::debug!("Failed to forward bus event: {}", e);
                            }
                        }
                        // Handle shutdown
                        _ = tokio::signal::ctrl_c() => {
                            tracing::info!("Worker bridge received shutdown signal");
                            break;
                        }
                    }
                }

                // Cleanup
                heartbeat_handle.abort();
                sse_handle.abort();
                tracing::info!("Worker bridge stopped");
            }
        });

        Ok(Some(TuiWorkerBridge {
            worker_id,
            worker_name,
            cmd_tx,
            task_rx,
            handle,
        }))
    }
}

/// Forward bus events to the A2A server for observability
async fn forward_bus_event(
    client: &Client,
    server: &str,
    token: &str,
    worker_id: &str,
    envelope: &BusEnvelope,
) -> Result<()> {
    // Only forward certain event types
    let payload = match &envelope.message {
        BusMessage::AgentReady {
            agent_id,
            capabilities,
        } => {
            serde_json::json!({
                "type": "agent_ready",
                "worker_id": worker_id,
                "agent_id": agent_id,
                "capabilities": capabilities,
            })
        }
        BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } => {
            serde_json::json!({
                "type": "task_update",
                "worker_id": worker_id,
                "task_id": task_id,
                "state": format!("{:?}", state),
                "message": message,
            })
        }
        BusMessage::AgentMessage { from, to, parts } => {
            let text = parts
                .iter()
                .filter_map(|p| {
                    if let crate::a2a::types::Part::Text { text } = p {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            serde_json::json!({
                "type": "agent_message",
                "worker_id": worker_id,
                "from": from,
                "to": to,
                "text": text,
            })
        }
        // Skip other event types for now
        _ => return Ok(()),
    };

    let url = format!("{}/v1/agent/workers/{}/events", server, worker_id);
    let res = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        tracing::debug!("Failed to forward bus event: {}", res.status());
    }

    Ok(())
}
