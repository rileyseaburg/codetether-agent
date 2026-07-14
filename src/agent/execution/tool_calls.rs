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

use super::tool_calls_policy::{blocked, can_parallelize};
use super::tool_result_record::record_results;
use crate::agent::Agent;
use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::ToolResult;

impl Agent {
    pub(super) async fn execute_tool_calls(
        &self,
        session: &mut Session,
        tool_calls: Vec<ContentPart>,
    ) {
        let results = if can_parallelize(session, &tool_calls) {
            super::tool_calls_parallel::execute(self, session, &tool_calls).await
        } else {
            self.execute_sequential(session, &tool_calls).await
        };
        record_results(session, tool_calls, results);
    }

    async fn execute_sequential(
        &self,
        session: &Session,
        calls: &[ContentPart],
    ) -> Vec<ToolResult> {
        let mut out = Vec::with_capacity(calls.len());
        for part in calls {
            if let Some(result) = blocked(session, part) {
                out.push(result);
                continue;
            }
            let Some(call) = part.as_tool_call() else {
                continue;
            };
            out.push(
                self.execute_tool_for_session(session, call.name, call.arguments)
                    .await,
            );
        }
        out
    }
}
