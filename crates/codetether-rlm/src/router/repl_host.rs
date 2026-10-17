//! Concrete [`RouterHost`] over [`RlmRepl`] and tool dispatch.

use crate::repl::RlmRepl;
use crate::router::{HostToolResult, RouterHost};
use crate::tools::{self, RlmToolResult};
use crate::traits::ToolDefinition;

/// Host adapter wrapping an [`RlmRepl`].
///
/// # Examples
/// ```
/// use codetether_rlm::{repl::{RlmRepl, ReplRuntime}, router::{ReplHost, RouterHost}};
/// let mut repl = RlmRepl::new("text".into(), ReplRuntime::Rust);
/// let host = ReplHost(&mut repl);
/// assert_eq!(host.tool_definitions().len(), 8);
/// ```
pub struct ReplHost<'a>(
    /// Mutable context interpreter used by dispatched operations.
    pub &'a mut RlmRepl,
);

impl RouterHost for ReplHost<'_> {
    fn tool_definitions(&self) -> Vec<ToolDefinition> {
        tools::rlm_tool_definitions()
            .into_iter()
            .map(|d| ToolDefinition {
                name: d.name,
                description: d.description,
                parameters: d.parameters,
            })
            .collect()
    }

    fn dispatch(&mut self, name: &str, arguments: &str) -> Option<HostToolResult> {
        tools::dispatch_tool_call(name, arguments, self.0).map(|r| match r {
            RlmToolResult::Output(o) => HostToolResult::Output(o),
            RlmToolResult::Final(a) => HostToolResult::Final(a),
        })
    }
}
