//! Forward retired goal-completion actions to the independent verifier.
use super::super::params::Params;
use anyhow::Result;
use codetether_tool_core::ToolResult;
pub(crate) async fn complete_goal<H: crate::Host>(params: Params) -> Result<ToolResult> {
    H::complete_goal(
        params.note.or(params.progress_note).unwrap_or_default(),
        params.ct_session_id,
        params.ct_current_model,
        params.ct_workspace,
    )
    .await
}
