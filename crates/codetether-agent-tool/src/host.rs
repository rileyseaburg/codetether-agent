//! Native action operations used by the shared agent tool.
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_agent_lifecycle::ResumeConfig;
use codetether_tool_core::ToolResult;
/// Application agent operations invoked after shared input validation.
/// # Implementors
/// Applications forward to their existing spawn, message and lifecycle services.
#[async_trait]
pub trait Host: Send + Sync + 'static {
    /// Hydrate the owner's durable child state before action routing.
    async fn hydrate(parent: Option<&str>) -> Result<()>;
    /// Spawn using the existing runtime context.
    async fn spawn(params: &Params) -> Result<ToolResult>;
    /// Send a message using the existing local/remote runtime.
    async fn message(params: &Params) -> Result<ToolResult>;
    /// Return local and discovered peer listings for the owner.
    async fn list(owner: Option<&str>) -> Result<ToolResult>;
    /// Return the current owner-scoped child status report.
    fn status(owner: Option<&str>) -> ToolResult;
    /// Interrupt a child, optionally recording the configured marker.
    async fn interrupt(target: &str, owner: Option<&str>, marker: bool) -> Result<ToolResult>;
    /// Close one child subtree while preserving durable identities.
    async fn close(target: &str, owner: Option<&str>) -> Result<ToolResult>;
    /// Resume a durable child with current parent settings.
    async fn resume(target: &str, owner: Option<&str>, config: ResumeConfig) -> Result<ToolResult>;
    /// Permanently remove a child after owner-scoped lookup.
    async fn kill(target: &str, owner: Option<&str>) -> Result<ToolResult>;
}
