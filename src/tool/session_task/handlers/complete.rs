//! Compatibility handling for the retired `complete_goal` action.

use super::super::params::Params;
use crate::tool::ToolResult;
use anyhow::Result;

pub(in crate::tool::goal::session_task) async fn complete_goal(
    params: Params,
) -> Result<ToolResult> {
    let args = crate::tool::goal::update::Args {
        status: "complete".to_string(),
        session_id: params.ct_session_id,
    };
    crate::tool::goal::update_run::run(args).await
}
