//! Message-part constructors shared by pairing-invariant tests.

use codetether_provider_core::{ContentPart, Message, Role};

pub(in crate::convert::repair::tests) fn call(id: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: id.into(),
        name: "bash".into(),
        arguments: "{}".into(),
        thought_signature: None,
    }
}

pub(in crate::convert::repair::tests) fn result(id: &str) -> ContentPart {
    ContentPart::ToolResult {
        tool_call_id: id.into(),
        content: "ok".into(),
    }
}

pub(in crate::convert::repair::tests) fn text(role: Role, body: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: body.into() }],
    }
}
