//! Identity-preserving recovery of durable sessions; never load-or-create.
use crate::session::Session;
use serde::Deserialize;
use serde_json::Value;
mod authority;
mod errors;
mod execution;
mod handler;
mod scope;
pub(super) use errors::http_error;
pub(super) use handler::handle;
#[cfg(test)]
mod tests;

/// Metadata and optional prompt supplied by the dashboard resume flow.
#[derive(Deserialize)]
pub(super) struct ResumeRequest {
    pub prompt: Option<String>,
    pub agent: Option<String>,
    pub model: Option<String>,
}

/// Resume only the requested durable identity, preserving all storage errors.
/// # Errors
/// Returns workspace-scope and durable storage failures without creating a session.
pub(super) async fn resume(
    id: &str,
    request: ResumeRequest,
    workspace: &std::path::Path,
) -> anyhow::Result<Value> {
    let recorded = scope::verify_session(id, workspace).await?;
    let mut session = Session::resume(id).await?;
    scope::verify_paths(
        session.metadata.directory.as_deref().unwrap_or(&recorded),
        workspace,
    )?;
    if let Some(agent) = request.agent {
        session.set_agent_name(agent);
    }
    if let Some(model) = request.model {
        session.metadata.model = Some(model);
    }
    execution::finish(session, request.prompt).await
}
