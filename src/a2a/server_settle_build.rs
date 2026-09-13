//! Constructors for the A2A task, reply message, and artifact of one turn.

use uuid::Uuid;

use crate::a2a::types::{
    Artifact, Message, MessageRole, MessageSendParams, Part, Task, TaskState, TaskStatus,
};

/// The task id (client-supplied or fresh) and its initial `Working` record.
pub(in crate::a2a) fn open_task(params: &MessageSendParams) -> (String, Task) {
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
        metadata: Default::default(),
    };
    (task_id, task)
}

pub(super) fn agent_message(task_id: &str, context_id: Option<&str>, text: String) -> Message {
    Message {
        message_id: Uuid::new_v4().to_string(),
        role: MessageRole::Agent,
        parts: vec![Part::Text { text }],
        context_id: context_id.map(ToString::to_string),
        task_id: Some(task_id.to_string()),
        metadata: Default::default(),
        extensions: vec![],
    }
}

pub(super) fn response_artifact(message: &Message) -> Artifact {
    Artifact {
        artifact_id: Uuid::new_v4().to_string(),
        parts: message.parts.clone(),
        name: Some("response".to_string()),
        description: None,
        metadata: Default::default(),
        extensions: vec![],
    }
}
