//! Integration tests for A2AClient
//!
//! These tests demonstrate real usage of A2AClient with a mock server.

use axum::{
    Json, Router,
    routing::{get, post},
};
use codetether_agent::a2a::{
    A2AClient,
    types::{
        AgentCapabilities, AgentCard, AgentSkill, JsonRpcRequest, JsonRpcResponse, Message,
        MessageRole, MessageSendParams, Part, Task, TaskState, TaskStatus,
    },
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Creates a mock A2A server for testing
async fn create_mock_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    // Agent card endpoint
    async fn agent_card() -> Json<AgentCard> {
        Json(AgentCard {
            name: "Test Agent".to_string(),
            description: "A test agent for integration tests".to_string(),
            url: "http://localhost:8080".to_string(),
            version: "1.0.0".to_string(),
            protocol_version: "0.3.0".to_string(),
            preferred_transport: None,
            additional_interfaces: vec![],
            capabilities: AgentCapabilities {
                streaming: false,
                push_notifications: false,
                state_transition_history: true,
                extensions: vec![],
            },
            skills: vec![AgentSkill {
                id: "test-skill".to_string(),
                name: "Test Skill".to_string(),
                description: "A test skill".to_string(),
                tags: vec!["test".to_string()],
                examples: vec!["Example 1".to_string()],
                input_modes: vec!["text".to_string()],
                output_modes: vec!["text".to_string()],
            }],
            default_input_modes: vec!["text".to_string()],
            default_output_modes: vec!["text".to_string()],
            provider: None,
            icon_url: None,
            documentation_url: None,
            security_schemes: Default::default(),
            security: vec![],
            supports_authenticated_extended_card: false,
            signatures: vec![],
        })
    }

    // JSON-RPC message endpoint
    async fn handle_rpc(Json(request): Json<JsonRpcRequest>) -> Json<JsonRpcResponse> {
        match request.method.as_str() {
            "message/send" => {
                let task = Task {
                    id: "task-123".to_string(),
                    context_id: Some("ctx-456".to_string()),
                    status: TaskStatus {
                        state: TaskState::Completed,
                        message: Some(Message {
                            message_id: "msg-789".to_string(),
                            role: MessageRole::Agent,
                            parts: vec![Part::Text {
                                text: "Hello from test agent!".to_string(),
                            }],
                            context_id: None,
                            task_id: Some("task-123".to_string()),
                            metadata: Default::default(),
                            extensions: vec![],
                        }),
                        timestamp: Some("2024-01-01T00:00:00Z".to_string()),
                    },
                    artifacts: vec![],
                    history: vec![],
                    metadata: Default::default(),
                };

                Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!(task)),
                    error: None,
                })
            }
            "tasks/get" => {
                let task = Task {
                    id: "task-123".to_string(),
                    context_id: Some("ctx-456".to_string()),
                    status: TaskStatus {
                        state: TaskState::Completed,
                        message: None,
                        timestamp: Some("2024-01-01T00:00:00Z".to_string()),
                    },
                    artifacts: vec![],
                    history: vec![],
                    metadata: Default::default(),
                };

                Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!(task)),
                    error: None,
                })
            }
            "tasks/cancel" => {
                let task = Task {
                    id: "task-123".to_string(),
                    context_id: Some("ctx-456".to_string()),
                    status: TaskStatus {
                        state: TaskState::Cancelled,
                        message: None,
                        timestamp: Some("2024-01-01T00:00:00Z".to_string()),
                    },
                    artifacts: vec![],
                    history: vec![],
                    metadata: Default::default(),
                };

                Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!(task)),
                    error: None,
                })
            }
            _ => Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(
                    codetether_agent::a2a::types::JsonRpcError::method_not_found(&request.method),
                ),
            }),
        }
    }

    let app = Router::new()
        .route("/.well-known/agent.json", get(agent_card))
        .route("/", post(handle_rpc));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (addr, handle)
}

#[tokio::test]
async fn test_a2a_client_get_agent_card() {
    // Start mock server
    let (addr, _server_handle) = create_mock_server().await;

    // Create A2A client
    let client = A2AClient::new(format!("http://{}", addr));

    // Get agent card
    let agent_card = client
        .get_agent_card()
        .await
        .expect("Failed to get agent card");

    // Verify response
    assert_eq!(agent_card.name, "Test Agent");
    assert_eq!(agent_card.version, "1.0.0");
    assert_eq!(agent_card.protocol_version, "0.3.0");
    assert_eq!(agent_card.skills.len(), 1);
    assert_eq!(agent_card.skills[0].id, "test-skill");
}

#[tokio::test]
async fn test_a2a_client_send_message() {
    // Start mock server
    let (addr, _server_handle) = create_mock_server().await;

    // Create A2A client
    let client = A2AClient::new(format!("http://{}", addr));

    // Create a message
    let message = Message {
        message_id: "msg-001".to_string(),
        role: MessageRole::User,
        parts: vec![Part::Text {
            text: "Hello, test agent!".to_string(),
        }],
        context_id: None,
        task_id: None,
        metadata: Default::default(),
        extensions: vec![],
    };

    // Send message
    let params = MessageSendParams {
        message,
        configuration: None,
    };

    let task = client
        .send_message(params)
        .await
        .expect("Failed to send message");

    // Verify response
    assert_eq!(task.id, "task-123");
    assert_eq!(task.status.state, TaskState::Completed);
    assert!(task.status.message.is_some());
}

#[tokio::test]
async fn test_a2a_client_get_task() {
    // Start mock server
    let (addr, _server_handle) = create_mock_server().await;

    // Create A2A client
    let client = A2AClient::new(format!("http://{}", addr));

    // Get task status
    let task = client
        .get_task("task-123", Some(10))
        .await
        .expect("Failed to get task");

    // Verify response
    assert_eq!(task.id, "task-123");
    assert_eq!(task.status.state, TaskState::Completed);
}

#[tokio::test]
async fn test_a2a_client_cancel_task() {
    // Start mock server
    let (addr, _server_handle) = create_mock_server().await;

    // Create A2A client
    let client = A2AClient::new(format!("http://{}", addr));

    // Cancel task
    let task = client
        .cancel_task("task-123")
        .await
        .expect("Failed to cancel task");

    // Verify response
    assert_eq!(task.id, "task-123");
    assert_eq!(task.status.state, TaskState::Cancelled);
}

#[tokio::test]
async fn test_a2a_client_with_token() {
    // Start mock server
    let (addr, _server_handle) = create_mock_server().await;

    // Create A2A client with auth token
    let client = A2AClient::new(format!("http://{}", addr)).with_token("test-token-123");

    // The token is used internally in call_rpc - this test verifies the builder pattern works
    let agent_card = client
        .get_agent_card()
        .await
        .expect("Failed to get agent card");
    assert_eq!(agent_card.name, "Test Agent");
}

#[tokio::test]
async fn test_a2a_client_raw_rpc_call() {
    // Start mock server
    let (addr, _server_handle) = create_mock_server().await;

    // Create A2A client
    let client = A2AClient::new(format!("http://{}", addr));

    // Make a raw RPC call
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(42),
        method: "message/send".to_string(),
        params: json!({
            "message": {
                "messageId": "raw-msg",
                "role": "user",
                "parts": [{"kind": "text", "text": "Raw RPC test"}]
            }
        }),
    };

    let response = client
        .call_rpc(request)
        .await
        .expect("Failed to make RPC call");

    // Verify response
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, json!(42));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[tokio::test]
async fn test_a2a_client_base_url_normalization() {
    // Test that trailing slashes are removed from base URL
    let client = A2AClient::new("http://example.com/");

    // The client should work with normalized URL
    // We can't easily test the internal state, but we verify the client is created
    // and the with_token builder works
    let _client_with_token = client.with_token("token");
}
