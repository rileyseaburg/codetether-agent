//! Load the exact calling session before bounded history browsing.
use crate::{BrowseSource, fault::fault_result, indexed, parse_browse_action};
use anyhow::Result;
use codetether_context_policy::fault::Fault;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<S: BrowseSource>(args: Value) -> Result<ToolResult> {
    let action = match parse_browse_action(&args) {
        Ok(a) => a,
        Err(e) => return Ok(ToolResult::error(e)),
    };
    let session = match codetether_context_loader::load_calling_window::<S>(&args, 0).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return Ok(fault_result(
                Fault::NoMatch,
                "No session found for this call.",
            ));
        }
        Err(e) => {
            return Ok(fault_result(
                Fault::BackendError {
                    reason: e.to_string(),
                },
                format!("failed to load session: {e}"),
            ));
        }
    };
    indexed::run::<S>(&session, action, &args).await
}
