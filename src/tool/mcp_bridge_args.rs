//! Argument shaping for MCP bridge calls.

use serde_json::{Value, json};

pub(super) fn normalized(arguments: Value) -> Value {
    if arguments.is_null() {
        json!({})
    } else {
        arguments
    }
}