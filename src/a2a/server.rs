//! A2A Server - serve as an A2A agent

use super::types::*;
use crate::session::{Session, SessionEvent};
use crate::telemetry::record_persistent;
use anyhow::Result;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use uuid::Uuid;

/// A2A Server state
#[derive(Clone)]
pub struct A2AServer {
    tasks: Arc<DashMap<String, Task>>,
    agent_card: AgentCard,
    /// Optional bus for emitting session events for SSE streaming
    bus: Option<Arc<crate::bus::AgentBus>>,
}

impl A2AServer {
    /// Create a new A2A server without a bus
    pub fn new(agent_card: AgentCard) -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
            agent_card,
            bus: None,
        }
    }

    /// Create a new A2A server with a bus for event streaming
    pub fn with_bus(agent_card: AgentCard, bus: Arc<crate::bus::AgentBus>) -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
            agent_card,
            bus: Some(bus),
        }
    }

    /// Create the router for A2A endpoints
    pub fn router(self) -> Router {
        Router::new()
            .route("/.well-known/agent.json", get(get_agent_card))
            .route("/.well-known/agent-card.json", get(get_agent_card))
            .route("/", post(handle_rpc))
            .with_state(self)
    }

    /// Get the agent card for this server
    #[allow(dead_code)]
    pub fn card(&self) -> &AgentCard {
        &self.agent_card
    }

    /// Create a default agent card
    pub fn default_card(url: &str) -> AgentCard {
        AgentCard {
            name: "CodeTether Agent".to_string(),
            description: "A2A-native AI coding agent for the CodeTether ecosystem".to_string(),
            url: url.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: "0.3.0".to_string(),
            preferred_transport: None,
            additional_interfaces: vec![],
            capabilities: AgentCapabilities {
                streaming: true,
                push_notifications: false,
                state_transition_history: true,
                extensions: vec![],
            },
            skills: vec![
                AgentSkill {
                    id: "code".to_string(),
                    name: "Code Generation".to_string(),
                    description: "Write, edit, and refactor code".to_string(),
                    tags: vec!["code".to_string(), "programming".to_string()],
                    examples: vec![
                        "Write a function to parse JSON".to_string(),
                        "Refactor this code to use async/await".to_string(),
                    ],
                    input_modes: vec!["text/plain".to_string()],
                    output_modes: vec!["text/plain".to_string()],
                },
                AgentSkill {
                    id: "debug".to_string(),
                    name: "Debugging".to_string(),
                    description: "Debug and fix code issues".to_string(),
                    tags: vec!["debug".to_string(), "fix".to_string()],
                    examples: vec![
                        "Why is this function returning undefined?".to_string(),
                        "Fix the null pointer exception".to_string(),
                    ],
                    input_modes: vec!["text/plain".to_string()],
                    output_modes: vec!["text/plain".to_string()],
                },
                AgentSkill {
                    id: "explain".to_string(),
                    name: "Code Explanation".to_string(),
                    description: "Explain code and concepts".to_string(),
                    tags: vec!["explain".to_string(), "learn".to_string()],
                    examples: vec![
                        "Explain how this algorithm works".to_string(),
                        "What does this regex do?".to_string(),
                    ],
                    input_modes: vec!["text/plain".to_string()],
                    output_modes: vec!["text/plain".to_string()],
                },
            ],
            default_input_modes: vec!["text/plain".to_string(), "application/json".to_string()],
            default_output_modes: vec!["text/plain".to_string(), "application/json".to_string()],
            provider: Some(AgentProvider {
                organization: "CodeTether".to_string(),
                url: "https://codetether.run".to_string(),
            }),
            icon_url: None,
            documentation_url: None,
            security_schemes: Default::default(),
            security: vec![],
            supports_authenticated_extended_card: false,
            signatures: vec![],
        }
    }
}

/// Get agent card handler
async fn get_agent_card(State(server): State<A2AServer>) -> Json<AgentCard> {
    Json(server.agent_card.clone())
}

fn record_a2a_message_telemetry(
    tool_name: &str,
    task_id: &str,
    blocking: bool,
    prompt: &str,
    duration: Duration,
    success: bool,
    output: Option<String>,
    error: Option<String>,
) {
    let record = crate::telemetry::A2AMessageRecord {
        tool_name: tool_name.to_string(),
        task_id: task_id.to_string(),
        blocking,
        prompt: prompt.to_string(),
        duration_ms: duration.as_millis() as u64,
        success,
        output,
        error,
        timestamp: chrono::Utc::now(),
    };
    let _ = record_persistent(
        "a2a_message",
        &serde_json::to_value(&record).unwrap_or_default(),
    );
}

/// Handle JSON-RPC requests
async fn handle_rpc(
    State(server): State<A2AServer>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, (StatusCode, Json<JsonRpcResponse>)> {
    let request_id = request.id.clone();
    let response = match request.method.as_str() {
        "message/send" => handle_message_send(&server, request).await,
        "message/stream" => handle_message_stream(&server, request).await,
        "tasks/get" => handle_tasks_get(&server, request).await,
        "tasks/cancel" => handle_tasks_cancel(&server, request).await,
        _ => Err(JsonRpcError::method_not_found(&request.method)),
    };

    match response {
        Ok(result) => Ok(Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request_id.clone(),
            result: Some(result),
            error: None,
        })),
        Err(error) => Err((
            StatusCode::OK,
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id,
                result: None,
                error: Some(error),
            }),
        )),
    }
}

async fn handle_message_send(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: MessageSendParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    // Create a new task
    let task_id = params
        .message
        .task_id
        .clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let task = Task {
        id: task_id.clone(),
        context_id: params.message.context_id.clone(),
        status: TaskStatus {
            state: TaskState::Working,
            message: Some(params.message.clone()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        },
        artifacts: vec![],
        history: vec![params.message.clone()],
        metadata: std::collections::HashMap::new(),
    };

    server.tasks.insert(task_id.clone(), task.clone());

    // Extract prompt text from message parts
    let prompt: String = params
        .message
        .parts
        .iter()
        .filter_map(|p| match p {
            Part::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    if prompt.is_empty() {
        // Update task to failed
        if let Some(mut t) = server.tasks.get_mut(&task_id) {
            t.status.state = TaskState::Failed;
            t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        }
        return Err(JsonRpcError::invalid_params("No text content in message"));
    }

    // Determine if blocking (default true for message/send)
    let blocking = params
        .configuration
        .as_ref()
        .and_then(|c| c.blocking)
        .unwrap_or(true);

    if blocking {
        // Synchronous execution: create session, run prompt, return completed task
        let mut session = Session::new().await.map_err(|e| {
            JsonRpcError::internal_error(format!("Failed to create session: {}", e))
        })?;
        let started_at = Instant::now();

        match session.prompt(&prompt).await {
            Ok(result) => {
                let result_text = result.text;
                let response_message = Message {
                    message_id: Uuid::new_v4().to_string(),
                    role: MessageRole::Agent,
                    parts: vec![Part::Text {
                        text: result_text.clone(),
                    }],
                    context_id: params.message.context_id.clone(),
                    task_id: Some(task_id.clone()),
                    metadata: std::collections::HashMap::new(),
                    extensions: vec![],
                };

                let artifact = Artifact {
                    artifact_id: Uuid::new_v4().to_string(),
                    parts: vec![Part::Text {
                        text: result_text.clone(),
                    }],
                    name: Some("response".to_string()),
                    description: None,
                    metadata: std::collections::HashMap::new(),
                    extensions: vec![],
                };

                if let Some(mut t) = server.tasks.get_mut(&task_id) {
                    t.status.state = TaskState::Completed;
                    t.status.message = Some(response_message.clone());
                    t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                    t.artifacts.push(artifact.clone());
                    t.history.push(response_message);

                    let status_event = TaskStatusUpdateEvent {
                        id: task_id.clone(),
                        status: t.status.clone(),
                        is_final: true,
                        metadata: std::collections::HashMap::new(),
                    };
                    let artifact_event = TaskArtifactUpdateEvent {
                        id: task_id.clone(),
                        artifact,
                        metadata: std::collections::HashMap::new(),
                    };
                    tracing::debug!(
                        task_id = %task_id,
                        event = ?StreamEvent::StatusUpdate(status_event),
                        "Task completed"
                    );
                    tracing::debug!(
                        task_id = %task_id,
                        event = ?StreamEvent::ArtifactUpdate(artifact_event),
                        "Artifact produced"
                    );
                }

                record_a2a_message_telemetry(
                    "a2a_message_send",
                    &task_id,
                    true,
                    &prompt,
                    started_at.elapsed(),
                    true,
                    Some(result_text),
                    None,
                );
            }
            Err(e) => {
                let error_message = Message {
                    message_id: Uuid::new_v4().to_string(),
                    role: MessageRole::Agent,
                    parts: vec![Part::Text {
                        text: format!("Error: {}", e),
                    }],
                    context_id: params.message.context_id.clone(),
                    task_id: Some(task_id.clone()),
                    metadata: std::collections::HashMap::new(),
                    extensions: vec![],
                };

                if let Some(mut t) = server.tasks.get_mut(&task_id) {
                    t.status.state = TaskState::Failed;
                    t.status.message = Some(error_message);
                    t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                }

                record_a2a_message_telemetry(
                    "a2a_message_send",
                    &task_id,
                    true,
                    &prompt,
                    started_at.elapsed(),
                    false,
                    None,
                    Some(e.to_string()),
                );
            }
        }
    } else {
        // Async execution: spawn background task, return immediately with Working state
        let tasks = server.tasks.clone();
        let context_id = params.message.context_id.clone();
        let spawn_task_id = task_id.clone();

        tokio::spawn(async move {
            let task_id = spawn_task_id;
            let started_at = Instant::now();
            let mut session = match Session::new().await {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to create session for task {}: {}", task_id, e);
                    if let Some(mut t) = tasks.get_mut(&task_id) {
                        t.status.state = TaskState::Failed;
                        t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                    }
                    record_a2a_message_telemetry(
                        "a2a_message_send",
                        &task_id,
                        false,
                        &prompt,
                        started_at.elapsed(),
                        false,
                        None,
                        Some(e.to_string()),
                    );
                    return;
                }
            };

            match session.prompt(&prompt).await {
                Ok(result) => {
                    let result_text = result.text;
                    let response_message = Message {
                        message_id: Uuid::new_v4().to_string(),
                        role: MessageRole::Agent,
                        parts: vec![Part::Text {
                            text: result_text.clone(),
                        }],
                        context_id,
                        task_id: Some(task_id.clone()),
                        metadata: std::collections::HashMap::new(),
                        extensions: vec![],
                    };

                    let artifact = Artifact {
                        artifact_id: Uuid::new_v4().to_string(),
                        parts: vec![Part::Text {
                            text: result_text.clone(),
                        }],
                        name: Some("response".to_string()),
                        description: None,
                        metadata: std::collections::HashMap::new(),
                        extensions: vec![],
                    };

                    if let Some(mut t) = tasks.get_mut(&task_id) {
                        t.status.state = TaskState::Completed;
                        t.status.message = Some(response_message.clone());
                        t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                        t.artifacts.push(artifact);
                        t.history.push(response_message);
                    }

                    record_a2a_message_telemetry(
                        "a2a_message_send",
                        &task_id,
                        false,
                        &prompt,
                        started_at.elapsed(),
                        true,
                        Some(result_text),
                        None,
                    );
                }
                Err(e) => {
                    tracing::error!("Task {} failed: {}", task_id, e);
                    if let Some(mut t) = tasks.get_mut(&task_id) {
                        t.status.state = TaskState::Failed;
                        t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                    }
                    record_a2a_message_telemetry(
                        "a2a_message_send",
                        &task_id,
                        false,
                        &prompt,
                        started_at.elapsed(),
                        false,
                        None,
                        Some(e.to_string()),
                    );
                }
            }
        });
    }

    // Return current task state wrapped in SendMessageResponse
    let task = server.tasks.get(&task_id).unwrap();
    let response = SendMessageResponse::Task(task.value().clone());
    serde_json::to_value(response)
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}

async fn handle_message_stream(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    // message/stream submits the task for async processing.
    // The client should poll tasks/get for status updates.
    // True SSE streaming requires a dedicated endpoint outside JSON-RPC.

    let params: MessageSendParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let task_id = params
        .message
        .task_id
        .clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let task = Task {
        id: task_id.clone(),
        context_id: params.message.context_id.clone(),
        status: TaskStatus {
            state: TaskState::Working,
            message: Some(params.message.clone()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        },
        artifacts: vec![],
        history: vec![params.message.clone()],
        metadata: std::collections::HashMap::new(),
    };

    server.tasks.insert(task_id.clone(), task.clone());

    // Extract prompt
    let prompt: String = params
        .message
        .parts
        .iter()
        .filter_map(|p| match p {
            Part::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    if prompt.is_empty() {
        if let Some(mut t) = server.tasks.get_mut(&task_id) {
            t.status.state = TaskState::Failed;
            t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        }
        return Err(JsonRpcError::invalid_params("No text content in message"));
    }

    // Spawn async processing with event streaming
    let tasks = server.tasks.clone();
    let context_id = params.message.context_id.clone();
    let spawn_task_id = task_id.clone();
    let bus = server.bus.clone();

    tokio::spawn(async move {
        let task_id = spawn_task_id;
        let started_at = Instant::now();

        // Create a channel for session events
        let (event_tx, mut event_rx) = mpsc::channel::<SessionEvent>(256);

        let mut session = match Session::new().await {
            Ok(s) => s,
            Err(e) => {
                tracing::error!(
                    "Failed to create session for stream task {}: {}",
                    task_id,
                    e
                );
                if let Some(mut t) = tasks.get_mut(&task_id) {
                    t.status.state = TaskState::Failed;
                    t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                }
                record_a2a_message_telemetry(
                    "a2a_message_stream",
                    &task_id,
                    false,
                    &prompt,
                    started_at.elapsed(),
                    false,
                    None,
                    Some(e.to_string()),
                );
                return;
            }
        };

        // Spawn a task to forward session events to the bus
        let bus_clone = bus.clone();
        let task_id_clone = task_id.clone();
        tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                let event_data = match &event {
                    SessionEvent::Thinking => {
                        serde_json::json!({ "type": "thinking" })
                    }
                    SessionEvent::ToolCallStart { name, arguments } => {
                        serde_json::json!({
                            "type": "tool_call_start",
                            "name": name,
                            "arguments": arguments
                        })
                    }
                    SessionEvent::ToolCallComplete {
                        name,
                        output,
                        success,
                    } => {
                        serde_json::json!({
                            "type": "tool_call_complete",
                            "name": name,
                            "output": output.chars().take(500).collect::<String>(),
                            "success": success
                        })
                    }
                    SessionEvent::TextChunk(text) => {
                        serde_json::json!({ "type": "text_chunk", "text": text })
                    }
                    SessionEvent::TextComplete(text) => {
                        serde_json::json!({ "type": "text_complete", "text": text })
                    }
                    SessionEvent::ThinkingComplete(thought) => {
                        serde_json::json!({ "type": "thinking_complete", "thought": thought })
                    }
                    SessionEvent::UsageReport {
                        prompt_tokens,
                        completion_tokens,
                        duration_ms,
                        model,
                    } => {
                        serde_json::json!({
                            "type": "usage_report",
                            "prompt_tokens": prompt_tokens,
                            "completion_tokens": completion_tokens,
                            "duration_ms": duration_ms,
                            "model": model
                        })
                    }
                    SessionEvent::Done => {
                        serde_json::json!({ "type": "done" })
                    }
                    SessionEvent::Error(err) => {
                        serde_json::json!({ "type": "error", "error": err })
                    }
                    SessionEvent::SessionSync(_) => {
                        continue; // Don't emit session sync to SSE
                    }
                };

                // Emit to bus for SSE subscribers
                if let Some(ref bus) = bus_clone {
                    let handle = bus.handle("a2a-stream");
                    handle.send(
                        format!("task.{}", task_id_clone),
                        crate::bus::BusMessage::TaskUpdate {
                            task_id: task_id_clone.clone(),
                            state: crate::a2a::types::TaskState::Working,
                            message: Some(serde_json::to_string(&event_data).unwrap_or_default()),
                        },
                    );
                }
            }
        });

        // Use prompt_with_events for streaming
        let registry = match crate::provider::ProviderRegistry::from_vault().await {
            Ok(r) => Arc::new(r),
            Err(e) => {
                tracing::error!("Failed to load provider registry: {}", e);
                if let Some(mut t) = tasks.get_mut(&task_id) {
                    t.status.state = TaskState::Failed;
                    t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                }
                return;
            }
        };

        match session
            .prompt_with_events(&prompt, event_tx, registry)
            .await
        {
            Ok(result) => {
                let result_text = result.text;
                let response_message = Message {
                    message_id: Uuid::new_v4().to_string(),
                    role: MessageRole::Agent,
                    parts: vec![Part::Text {
                        text: result_text.clone(),
                    }],
                    context_id,
                    task_id: Some(task_id.clone()),
                    metadata: std::collections::HashMap::new(),
                    extensions: vec![],
                };

                let artifact = Artifact {
                    artifact_id: Uuid::new_v4().to_string(),
                    parts: vec![Part::Text {
                        text: result_text.clone(),
                    }],
                    name: Some("response".to_string()),
                    description: None,
                    metadata: std::collections::HashMap::new(),
                    extensions: vec![],
                };

                if let Some(mut t) = tasks.get_mut(&task_id) {
                    t.status.state = TaskState::Completed;
                    t.status.message = Some(response_message.clone());
                    t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                    t.artifacts.push(artifact.clone());
                    t.history.push(response_message);

                    // Emit streaming events for SSE consumers
                    let status_event = TaskStatusUpdateEvent {
                        id: task_id.clone(),
                        status: t.status.clone(),
                        is_final: true,
                        metadata: std::collections::HashMap::new(),
                    };
                    let artifact_event = TaskArtifactUpdateEvent {
                        id: task_id.clone(),
                        artifact,
                        metadata: std::collections::HashMap::new(),
                    };
                    tracing::debug!(
                        task_id = %task_id,
                        event = ?StreamEvent::StatusUpdate(status_event),
                        "Task completed"
                    );
                    tracing::debug!(
                        task_id = %task_id,
                        event = ?StreamEvent::ArtifactUpdate(artifact_event),
                        "Artifact produced"
                    );
                }

                record_a2a_message_telemetry(
                    "a2a_message_stream",
                    &task_id,
                    false,
                    &prompt,
                    started_at.elapsed(),
                    true,
                    Some(result_text),
                    None,
                );
            }
            Err(e) => {
                tracing::error!("Stream task {} failed: {}", task_id, e);
                if let Some(mut t) = tasks.get_mut(&task_id) {
                    t.status.state = TaskState::Failed;
                    t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
                }
                record_a2a_message_telemetry(
                    "a2a_message_stream",
                    &task_id,
                    false,
                    &prompt,
                    started_at.elapsed(),
                    false,
                    None,
                    Some(e.to_string()),
                );
            }
        }
    });

    // Return task in Working state — client polls tasks/get for completion
    let response = SendMessageResponse::Task(task);
    serde_json::to_value(response)
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}

async fn handle_tasks_get(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: TaskQueryParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let task = server.tasks.get(&params.id).ok_or_else(|| JsonRpcError {
        code: TASK_NOT_FOUND,
        message: format!("Task not found: {}", params.id),
        data: None,
    })?;

    serde_json::to_value(task.value().clone())
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}

async fn handle_tasks_cancel(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: TaskQueryParams = serde_json::from_value(request.params)
        .map_err(|e| JsonRpcError::invalid_params(format!("Invalid parameters: {}", e)))?;

    let mut task = server
        .tasks
        .get_mut(&params.id)
        .ok_or_else(|| JsonRpcError {
            code: TASK_NOT_FOUND,
            message: format!("Task not found: {}", params.id),
            data: None,
        })?;

    if !task.status.state.is_active() {
        return Err(JsonRpcError {
            code: TASK_NOT_CANCELABLE,
            message: "Task is already in a terminal state".to_string(),
            data: None,
        });
    }

    task.status.state = TaskState::Cancelled;
    task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());

    serde_json::to_value(task.value().clone())
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {}", e)))
}
