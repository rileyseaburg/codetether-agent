//! Shared original parsing-test fixtures.
//! Argument-free tool calls must not be reported as truncated.

use crate::parse_tool_calls;

use codetether_provider_core::ContentPart;

pub(super) fn call(name: &str, arguments: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: format!("id-{name}"),
        name: name.into(),
        arguments: arguments.into(),
        thought_signature: None,
    }
}
