//! Tool-call replay for agent execution.
//!
//! This module applies provider-requested tool calls to the session and stores
//! the resulting tool history. When the LLM emits multiple calls in one turn
//! and **all** of them are in the read-only allowlist
//! ([`crate::tool::readonly::is_read_only`]), they are dispatched concurrently
//! via `join_all`; otherwise they run sequentially to preserve ordering
//! semantics for tools with side effects.
//!
//! # Examples
//!
//! ```ignore
//! agent.execute_tool_calls(&mut session, vec![]).await;
//! ```

use super::messages::PendingToolCall;
use super::tool_result_record::record_results;
use crate::agent::Agent;
use crate::session::Session;
use crate::tool::ToolResult;
use crate::tool::readonly::is_read_only;

impl Agent {
    pub(super) async fn execute_tool_calls(
        &self,
        session: &mut Session,
        tool_calls: Vec<PendingToolCall>,
    ) {
        let results = if can_parallelize(&tool_calls) {
            self.execute_parallel(&tool_calls).await
        } else {
            self.execute_sequential(&tool_calls).await
        };
        record_results(session, tool_calls, results);
    }

    async fn execute_sequential(&self, calls: &[PendingToolCall]) -> Vec<ToolResult> {
        let mut out = Vec::with_capacity(calls.len());
        for (_, name, arguments) in calls {
            out.push(self.execute_tool(name, arguments).await);
        }
        out
    }

    async fn execute_parallel(&self, calls: &[PendingToolCall]) -> Vec<ToolResult> {
        let futures = calls
            .iter()
            .map(|(_, name, arguments)| self.execute_tool(name, arguments));
        futures::future::join_all(futures).await
    }
}

fn can_parallelize(calls: &[PendingToolCall]) -> bool {
    calls.len() > 1 && calls.iter().all(|(_, name, _)| is_read_only(name))
}
