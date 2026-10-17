//! Build the initial working task record.
use super::*;
/// Build an initial working task from an accepted stream request.
///
/// # Arguments
///
/// * `params` - Accepted message request from which to build the task.
///
/// # Returns
///
/// The existing or generated task id and a new working task record.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::task_state::stream::open::open;
/// let _operation = open;
/// ```
pub fn open(params: &MessageSendParams) -> (String, Task) {
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

    (task_id, task)
}
