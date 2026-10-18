//! Preserve canonical task journal paths and independent goal verification.
use crate::{Host, VerifierFactory};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use std::path::PathBuf;
#[async_trait::async_trait]
impl<F: VerifierFactory> codetether_session_task::Host for Host<F> {
    fn log_path(session_id: &str) -> Result<PathBuf> {
        codetether_native_goals::task_log_path(session_id)
    }
    async fn complete_goal(
        evidence: String,
        session_id: Option<String>,
        current_model: Option<String>,
        workspace: Option<String>,
    ) -> Result<ToolResult> {
        let args = codetether_goal_tools::UpdateArgs {
            status: "complete".to_string(),
            evidence,
            session_id,
            current_model,
            workspace,
        };
        codetether_goal_tools::update::<Self>(args).await
    }
}
