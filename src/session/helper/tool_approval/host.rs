//! Connect shared approval decisions to native application services.
use crate::approval::{LiveApprovalDecision, LiveApprovalRequest};
use crate::session::SessionEvent;
use crate::tool::ToolResult;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::sync::mpsc;

pub(super) struct NativeHost<'a> {
    pub event_tx: &'a mpsc::Sender<SessionEvent>,
}

#[async_trait]
impl codetether_tool_approval::Host for NativeHost<'_> {
    async fn preflight(
        &self,
        workspace: &Path,
        tool: &str,
        args: &Value,
    ) -> (Option<ToolResult>, Vec<String>) {
        super::preflight::inspect(workspace, tool, args).await
    }
    async fn policy(&self, tool: &str, args: &Value) -> Option<ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(tool, args).await
    }
    async fn request(&self, request: LiveApprovalRequest) -> LiveApprovalDecision {
        crate::approval::live::request(self.event_tx, request).await
    }
}
