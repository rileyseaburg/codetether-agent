//! Application services required by the approval decision flow.
use async_trait::async_trait;
use codetether_approval_core::interaction::{LiveApprovalDecision, LiveApprovalRequest};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::Path;

/// Supplies diagnostics, policy evaluation and interactive decisions.
///
/// # Implementors
/// Applications connect their diagnostic manager, policy engine and approval UI.
#[async_trait]
pub trait Host: Sync {
    /// Check proposed edits, returning an optional block and verification warnings.
    async fn preflight(
        &self,
        workspace: &Path,
        tool: &str,
        args: &Value,
    ) -> (Option<ToolResult>, Vec<String>);
    /// Evaluate invocation policy; None permits execution without a prompt.
    async fn policy(&self, tool: &str, args: &Value) -> Option<ToolResult>;
    /// Present a request and wait for the user's approval, revision or denial.
    async fn request(&self, request: LiveApprovalRequest) -> LiveApprovalDecision;
}
