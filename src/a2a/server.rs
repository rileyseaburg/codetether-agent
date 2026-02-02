//! A2A Server - serve as an A2A agent

use super::types::*;
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

/// A2A Server state
#[derive(Clone)]
pub struct A2AServer {
    tasks: Arc<DashMap<String, Task>>,
    agent_card: AgentCard,
}

impl A2AServer {
    /// Create a new A2A server
    pub fn new(agent_card: AgentCard) -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
            agent_card,
        }
    }

    /// Create the router for A2A endpoints
    pub fn router(self) -> Router {
        Router::new()
            .route("/.well-known/agent.json", get(get_agent_card))
            .route("/", post(handle_rpc))
            .with_state(self)
    }

    /// Get the agent card for this server
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
            capabilities: AgentCapabilities {
                streaming: true,
                push_notifications: false,
                state_transition_history: true,
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
                url: "https://codetether.ai".to_string(),
            }),
            icon_url: None,
            documentation_url: None,
        }
    }
}

/// Get agent card handler
async fn get_agent_card(State(server): State<A2AServer>) -> Json<AgentCard> {
    Json(server.agent_card.clone())
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
        _ => Err(JsonRpcError {
            code: METHOD_NOT_FOUND,
            message: format!("Method not found: {}", request.method),
            data: None,
        }),
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
    let params: MessageSendParams = serde_json::from_value(request.params).map_err(|e| JsonRpcError {
        code: INVALID_PARAMS,
        message: format!("Invalid parameters: {}", e),
        data: None,
    })?;

    // Create a new task
    let task_id = params.message.task_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    
    let task = Task {
        id: task_id.clone(),
        context_id: params.message.context_id.clone(),
        status: TaskStatus {
            state: TaskState::Submitted,
            message: Some(params.message.clone()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        },
        artifacts: vec![],
        history: vec![params.message],
        metadata: std::collections::HashMap::new(),
    };

    server.tasks.insert(task_id.clone(), task.clone());

    // TODO: Process the task asynchronously

    serde_json::to_value(task).map_err(|e| JsonRpcError {
        code: INTERNAL_ERROR,
        message: format!("Serialization error: {}", e),
        data: None,
    })
}

async fn handle_message_stream(
    _server: &A2AServer,
    _request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    // TODO: Implement streaming
    Err(JsonRpcError {
        code: UNSUPPORTED_OPERATION,
        message: "Streaming not yet implemented".to_string(),
        data: None,
    })
}

async fn handle_tasks_get(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: TaskQueryParams = serde_json::from_value(request.params).map_err(|e| JsonRpcError {
        code: INVALID_PARAMS,
        message: format!("Invalid parameters: {}", e),
        data: None,
    })?;

    let task = server.tasks.get(&params.id).ok_or(JsonRpcError {
        code: TASK_NOT_FOUND,
        message: format!("Task not found: {}", params.id),
        data: None,
    })?;

    serde_json::to_value(task.value().clone()).map_err(|e| JsonRpcError {
        code: INTERNAL_ERROR,
        message: format!("Serialization error: {}", e),
        data: None,
    })
}

async fn handle_tasks_cancel(
    server: &A2AServer,
    request: JsonRpcRequest,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: TaskQueryParams = serde_json::from_value(request.params).map_err(|e| JsonRpcError {
        code: INVALID_PARAMS,
        message: format!("Invalid parameters: {}", e),
        data: None,
    })?;

    let mut task = server.tasks.get_mut(&params.id).ok_or(JsonRpcError {
        code: TASK_NOT_FOUND,
        message: format!("Task not found: {}", params.id),
        data: None,
    })?;

    if task.status.state.is_terminal() {
        return Err(JsonRpcError {
            code: TASK_NOT_CANCELABLE,
            message: "Task is already in a terminal state".to_string(),
            data: None,
        });
    }

    task.status.state = TaskState::Cancelled;
    task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());

    serde_json::to_value(task.value().clone()).map_err(|e| JsonRpcError {
        code: INTERNAL_ERROR,
        message: format!("Serialization error: {}", e),
        data: None,
    })
}
