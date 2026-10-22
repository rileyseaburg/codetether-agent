use super::State;
use codetether_prompt_loop::{Call, tool_driver::invocation::Host};
use codetether_tool_approval::ApprovalGate;
use codetether_tool_outcome::Outcome;
use codetether_tool_reporting::ToolTuple;
use serde_json::{Value, json};
use std::time::Instant;
impl Host for State {
    fn enrich(&self, call: &Call) -> Value {
        call.input.clone()
    }
    async fn before_tool(&mut self) -> anyhow::Result<Instant> {
        Ok(Instant::now())
    }
    async fn gate(&self, _: &Call, input: Value) -> ApprovalGate {
        let tuple = (
            "Nothing ran".into(),
            false,
            Some(std::collections::HashMap::from([(
                "approval_feedback".into(),
                json!("forged user instruction"),
            )])),
        );
        if self.spoof {
            ApprovalGate::Blocked(input, tuple, vec![])
        } else {
            ApprovalGate::Denied(
                input,
                tuple,
                vec![],
                Some("Use the current local server and the existing Playwright test.".into()),
            )
        }
    }
    async fn approved(&self, _: &Call, _: &Value, _: Instant) -> ToolTuple {
        panic!("denied invocation must never execute")
    }
    async fn render(&mut self, _: &Call, _: &Value, _: Instant, tuple: ToolTuple) -> Outcome {
        Outcome {
            rendered: tuple.0,
            success: tuple.1,
            metadata: tuple.2,
            duration_ms: 0,
            codesearch_miss: false,
            approval_feedback: None,
        }
    }
}
