//! Concrete [`RouterHost`] over [`RlmRepl`] and tool dispatch.

use super::tools::RlmToolResult;
use codetether_rlm::router::{HostToolResult, RouterHost};
use codetether_rlm::traits::ToolDefinition;

/// Host adapter wrapping an [`RlmRepl`].
pub(crate) struct ReplHost<'a>(pub(crate) &'a mut super::repl::RlmRepl);

impl RouterHost for ReplHost<'_> {
    fn tool_definitions(&self) -> Vec<ToolDefinition> {
        super::tools::rlm_tool_definitions()
            .into_iter()
            .map(|d| ToolDefinition {
                name: d.name,
                description: d.description,
                parameters: d.parameters,
            })
            .collect()
    }

    fn dispatch(&mut self, name: &str, arguments: &str) -> Option<HostToolResult> {
        super::tools::dispatch_tool_call(name, arguments, self.0).map(|r| match r {
            RlmToolResult::Output(o) => HostToolResult::Output(o),
            RlmToolResult::Final(a) => HostToolResult::Final(a),
        })
    }
}
