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

use super::tool_result_record::record_results;
use crate::agent::Agent;
use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::ToolResult;
use crate::tool::readonly::is_read_only;

impl Agent {
    pub(super) async fn execute_tool_calls(
        &self,
        session: &mut Session,
        tool_calls: Vec<ContentPart>,
    ) {
        let results = if can_parallelize(&tool_calls) {
            self.execute_parallel(&tool_calls).await
        } else {
            self.execute_sequential(&tool_calls).await
        };
        record_results(session, tool_calls, results);
    }

    async fn execute_sequential(&self, calls: &[ContentPart]) -> Vec<ToolResult> {
        let mut out = Vec::with_capacity(calls.len());
        for part in calls {
            let Some(call) = part.as_tool_call() else {
                continue;
            };
            out.push(self.execute_tool(call.name, call.arguments).await);
        }
        out
    }

    async fn execute_parallel(&self, calls: &[ContentPart]) -> Vec<ToolResult> {
        let futures = calls.iter().filter_map(|part| {
            let call = part.as_tool_call()?;
            Some(self.execute_tool(call.name, call.arguments))
        });
        futures::future::join_all(futures).await
    }
}

fn can_parallelize(calls: &[ContentPart]) -> bool {
    calls.len() > 1
        && calls
            .iter()
            .filter_map(ContentPart::as_tool_call)
            .all(|c| is_read_only(c.name))
}
