//! Start one structured agent turn in a session's active workspace.

use crate::mux::protocol::AgentResponse;

use super::context::ServerContext;

pub(super) async fn apply(
    context: &ServerContext,
    session: &str,
    task_id: String,
    prompt: String,
    session_id: Option<String>,
    max_steps: usize,
    tool_profile: Option<String>,
) -> AgentResponse {
    let workspace = context
        .state
        .read()
        .await
        .session(session)
        .and_then(|item| item.active())
        .map(|window| window.workspace.clone());
    let Some(workspace) = workspace else {
        return error(
            task_id,
            anyhow::anyhow!("mux session has no active workspace"),
        );
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
            session,
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
