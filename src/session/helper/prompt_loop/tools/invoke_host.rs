//! Session-specific effects for the shared invocation sequence.
use super::{Call, Runner, approved};
use codetether_tool_approval::ApprovalGate;
use codetether_tool_outcome::Outcome;
use codetether_tool_reporting::ToolTuple;
use serde_json::Value;
use std::time::Instant;

impl codetether_prompt_loop::tool_driver::invocation::Host for Runner<'_> {
    fn enrich(&self, call: &Call) -> Value {
        crate::session::helper::runtime::enrich_tool_input_for_turn(
            &call.input,
            &self.workspace.cwd,
            self.session,
            &self.lease_owner,
        )
    }
    async fn before_tool(&mut self) -> anyhow::Result<Instant> {
        crate::session::helper::persist::before_tool(self.session).await
    }
    async fn gate(&self, call: &Call, input: Value) -> ApprovalGate {
        if let Some(events) = &self.events {
            crate::session::helper::tool_approval::gate(
                &self.workspace.cwd,
                events,
                &call.id,
                &call.name,
                input,
            )
            .await
        } else {
            ApprovalGate::Ready(input, Vec::new())
        }
    }
    async fn approved(&self, call: &Call, input: &Value, started: Instant) -> ToolTuple {
        approved::run(self, call, input, started).await
    }
    async fn render(
        &mut self,
        call: &Call,
        input: &Value,
        started: Instant,
        tuple: ToolTuple,
    ) -> Outcome {
        super::super::outcome::render(self, call, input, started, tuple).await
    }
}
