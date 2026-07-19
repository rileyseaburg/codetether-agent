//! Resolves a claimed task's persisted or reusable conversation session.

use anyhow::{Context, Result};

use crate::session::Session;

use super::TaskContext;

pub(super) async fn load(context: &TaskContext) -> Result<Session> {
    load_ids(
        context.resume_session_id.as_deref(),
        context.context_id.as_deref(),
        context.preserve_session_workspace,
    )
    .await
}

async fn load_ids(
    resume_session_id: Option<&str>,
    context_id: Option<&str>,
    require_resume: bool,
) -> Result<Session> {
    if let Some(session_id) = resume_session_id {
        match Session::load(session_id).await {
            Ok(session) => return Ok(session),
            Err(error) if require_resume => {
                return Err(error).context("Verified author session is unavailable");
            }
            Err(_) => {}
        }
    }
    crate::a2a::session_resolve::resolve_session(context_id).await
}

#[cfg(test)]
#[path = "task_session_load_tests.rs"]
mod tests;
