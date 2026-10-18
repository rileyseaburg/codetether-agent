//! Native registry and policy adapter for shared tool execution.
#[path = "tool_exec_host.rs"]
mod host;
#[cfg(test)]
use crate::session::SessionEvent;
#[cfg(test)]
use crate::tool::ToolRegistry;
#[cfg(test)]
use tokio::sync::mpsc;
#[cfg(test)]
pub(in crate::session::helper) async fn execute_tool(
    tool_registry: &ToolRegistry,
    tool_name: &str,
    exec_input: &serde_json::Value,
    session_id: &str,
    exec_start: std::time::Instant,
    progress: Option<(&mpsc::Sender<SessionEvent>, &str)>,
) -> super::tool_policy::ToolTuple {
    let host = host::NativeHost {
        registry: tool_registry,
    };
    codetether_tool_execution::execute(
        &host, tool_name, exec_input, session_id, exec_start, progress,
    )
    .await
}

pub(in crate::session::helper) use host::NativeHost;
