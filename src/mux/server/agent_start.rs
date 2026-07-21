//! Start one structured agent turn in the mux workspace.

use crate::mux::protocol::AgentResponse;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &ServerContext,
    task_id: String,
    prompt: String,
    session_id: Option<String>,
    max_steps: usize,
    tool_profile: Option<String>,
) -> AgentResponse {
    let state = context.state.read().await;
    let workspace = state
        .windows
        .iter()
        .find(|window| window.id == state.active_window)
        .map(|window| window.workspace.clone());
    let mux_name = state.name.clone();
    drop(state);
    let Some(workspace) = workspace else {
        return error(task_id, anyhow::anyhow!("mux has no active workspace"));
    };
    context
        .tasks
        .start(
            &task_id,
            &prompt,
            session_id.as_deref(),
            max_steps,
            tool_profile.as_deref(),
            &workspace,
            &mux_name,
        )
        .map(|()| AgentResponse::Accepted {
            task_id: task_id.clone(),
        })
        .unwrap_or_else(|failure| error(task_id, failure))
}

fn error(task_id: String, error: anyhow::Error) -> AgentResponse {
    AgentResponse::Error {
        task_id: Some(task_id),
        message: error.to_string(),
    }
}
