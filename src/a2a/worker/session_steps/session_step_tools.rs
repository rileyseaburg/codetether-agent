use std::{path::Path, sync::Arc};

use crate::session::Session;

#[cfg(test)]
mod denied_tests;
mod host;
#[cfg(test)]
mod image_fixture;
#[cfg(test)]
mod image_tests;
mod tool_run;
use super::session_output;
pub(super) use codetether_a2a_worker_core::session_protocol::ToolCall;

pub(super) async fn execute_tool_call(
    session: &mut Session,
    registry: &crate::tool::ToolRegistry,
    auto_approve: super::super::AutoApprove,
    workspace_dir: &Path,
    model: &str,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    (tool_id, tool_name, tool_input): ToolCall,
) {
    codetether_a2a_worker_core::session_protocol::tool_call::execute_tool_call(
        &mut host::ApplicationHost {
            session,
            registry,
            workspace_dir,
            model,
            cb,
        },
        auto_approve,
        cb,
        (tool_id, tool_name, tool_input),
    )
    .await;
}
