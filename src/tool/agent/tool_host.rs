//! Forward shared agent-tool actions to existing application services.
use crate::tool::ToolResult;
use crate::tool::agent::{handlers, message, persistence, spawn, status, thread_lifecycle};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_agent_lifecycle::ResumeConfig;
use codetether_agent_tool::Host;
/// Application runtime used by the shared agent tool.
pub struct ApplicationHost;
#[async_trait]
impl Host for ApplicationHost {
    async fn hydrate(parent: Option<&str>) -> Result<()> {
        persistence::hydrate_parent(parent).await
    }
    async fn spawn(params: &Params) -> Result<ToolResult> {
        spawn::handle_spawn(params).await
    }
    async fn message(params: &Params) -> Result<ToolResult> {
        message::handle_message(params).await
    }
    async fn list(owner: Option<&str>) -> Result<ToolResult> {
        handlers::handle_list(owner).await
    }
    fn status(owner: Option<&str>) -> ToolResult {
        status::handle_status(owner)
    }
    async fn interrupt(target: &str, owner: Option<&str>, marker: bool) -> Result<ToolResult> {
        handlers::handle_interrupt(target, owner, marker).await
    }
    async fn close(target: &str, owner: Option<&str>) -> Result<ToolResult> {
        thread_lifecycle::close(target, owner).await
    }
    async fn resume(target: &str, owner: Option<&str>, config: ResumeConfig) -> Result<ToolResult> {
        thread_lifecycle::resume(target, owner, config).await
    }
    async fn kill(target: &str, owner: Option<&str>) -> Result<ToolResult> {
        handlers::handle_kill(target, owner).await
    }
}
