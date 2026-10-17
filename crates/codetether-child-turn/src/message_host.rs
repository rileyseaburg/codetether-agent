//! Native local/remote routing and message-task preparation.
use crate::{MailboxHost, Started, TurnHost};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_collaboration_input::MessageImage;
use codetether_tool_core::ToolResult;
/// Native identity routing and startup for shared message coordination.
/// # Implementors
/// Implement existing turn/mailbox interfaces and forward native message setup.
#[async_trait]
pub trait MessageHost: TurnHost + MailboxHost {
    /// Resolve a target into canonical child ID and display name in the owner scope.
    fn resolve(target: &str, owner: Option<&str>) -> Option<(String, String)>;
    /// Try the existing remote fallback when local identity resolution fails.
    async fn remote(target: &str, message: &str, params: &Params) -> Result<ToolResult>;
    /// Prepare and start one native message turn, preserving setup errors.
    async fn begin_message(
        agent_id: &str,
        message: String,
        images: Vec<MessageImage>,
        params: &Params,
    ) -> Result<Started<Self::SourceEvent, Self::Session>>;
}
