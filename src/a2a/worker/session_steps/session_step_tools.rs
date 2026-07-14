use std::{path::Path, sync::Arc};

use crate::session::Session;

#[path = "session_step_tools/collect.rs"]
mod collect;
mod tool_run;
use super::session_output;
pub(super) use collect::{ToolCall, collect_tool_calls};

pub(super) async fn execute_tool_call(
    session: &mut Session,
    registry: &crate::tool::ToolRegistry,
    auto_approve: super::super::AutoApprove,
    workspace_dir: &Path,
    model: &str,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    (tool_id, tool_name, tool_input): ToolCall,
) {
    if let Some(cb) = cb {
        cb(format!("[tool:start:{}]", tool_name));
    }
    if !super::super::is_tool_allowed(&tool_name, auto_approve) {
        session_output::add_tool_result(
            session,
            tool_id,
            format!(
                "Tool '{}' requires approval but auto-approve policy is {:?}",
                tool_name, auto_approve
            ),
        );
        return;
    }
    let input = super::super::enrich_tool_input_for_session_model(
        &tool_input,
        workspace_dir,
        session,
        Some(model),
    );
    let output = tool_run::run_tool(registry, &tool_name, input, cb).await;
    session_output::add_tool_result(session, tool_id, output);
}

pub(super) use session_output::append_text_output;
