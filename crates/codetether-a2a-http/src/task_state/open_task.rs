//! open task task operation.
use super::*;

/// The task id (client-supplied or fresh) and its initial `Working` record.
pub fn open_task(params: &MessageSendParams) -> (String, Task) {
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
