//! Short-circuit handler for inbound mesh introductions.
//!
//! Completes the task with a canned acknowledgement: no session is
//! created, no LLM call is made, nothing is persisted to disk. This is
//! what stops one intro per peer-restart from costing a completion and
//! a junk session file (2,459 observed before this fix).
//!
//! [`handle_intro`] completes `task` immediately with a static intro
//! acknowledgement.
use super::{A2AServer, emit_a2a_inbound, emit_a2a_outbound};
use crate::a2a::intro::reply::intro_ack_text;
use crate::a2a::types::{
    Artifact, JsonRpcError, Message, MessageRole, Part, SendMessageResponse, Task, TaskState,
    TaskStatus,
};
use uuid::Uuid;
pub(super) fn handle_intro(
    server: &A2AServer,
    task_id: &str,
    inbound: &Message,
    mut task: Task,
) -> Result<serde_json::Value, JsonRpcError> {
    let ack = intro_ack_text(&server.agent_card);
    let artifact_id = Uuid::new_v4().to_string();
    let response_message = Message {
        message_id: artifact_id.clone(),
        role: MessageRole::Agent,
        parts: vec![Part::Text { text: ack.clone() }],
        context_id: inbound.context_id.clone(),
        task_id: Some(task_id.to_string()),
        metadata: std::collections::HashMap::new(),
        extensions: vec![],
    };
    task.status = TaskStatus {
        state: TaskState::Completed,
        message: Some(response_message.clone()),
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    };
    task.artifacts.push(Artifact {
        artifact_id,
        parts: vec![Part::Text { text: ack }],
        name: Some("intro-ack".to_string()),
        description: None,
        metadata: std::collections::HashMap::new(),
        extensions: vec![],
    });
    task.history.push(response_message.clone());
    server.tasks.insert(task_id.to_string(), task.clone());
    emit_a2a_inbound(server, task_id, inbound);
    emit_a2a_outbound(server, task_id, &response_message);
    tracing::info!(task_id = %task_id, "A2A intro short-circuited (no LLM call)");
    serde_json::to_value(SendMessageResponse::Task(task))
        .map_err(|e| JsonRpcError::internal_error(format!("Serialization error: {e}")))
}
