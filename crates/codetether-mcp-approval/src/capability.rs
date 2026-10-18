//! Approval methods advertised during MCP initialization.
use serde_json::{Value, json};

pub(super) fn value() -> Option<Value> {
    Some(json!({
        "approvals": {
            "request": "approvals/request",
            "decision": "approvals/decision"
        }
    }))
}
