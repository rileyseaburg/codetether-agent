//! Execute shared child-tool requests with the canonical native agent tool.
use crate::{
    session::Session,
    tool::{Tool, ToolResult, agent::AgentTool},
};
use codetether_agent_command::request;
use std::path::Path;
struct Native;
impl request::Host for Native {
    async fn execute(params: serde_json::Value) -> anyhow::Result<ToolResult> {
        AgentTool::new().execute(params).await
    }
}
pub(crate) async fn spawn(
    session: &Session,
    cwd: &Path,
    name: &str,
    instructions: &str,
) -> ToolResult {
    request::spawn::<Native>(
        &session.id,
        session.metadata.model.as_deref(),
        cwd,
        name,
        instructions,
    )
    .await
}
pub(crate) async fn message(parent: &str, name: &str, message: &str) -> ToolResult {
    request::message::<Native>(parent, name, message).await
}
pub(crate) async fn kill(parent: &str, name: &str) -> ToolResult {
    request::kill::<Native>(parent, name).await
}
