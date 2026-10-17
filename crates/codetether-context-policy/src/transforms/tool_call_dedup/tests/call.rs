//! call.
use super::*;

pub(super) fn call(id: &str, name: &str, args: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: id.into(),
            name: name.into(),
            arguments: args.into(),
            thought_signature: None,
        }],
    }
}
