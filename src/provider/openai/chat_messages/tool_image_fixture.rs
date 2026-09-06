//! Equivalent packed and separate tool-result histories.

use super::super::support::{image, reply, text};
use crate::provider::{ContentPart, Message, Role};

pub(super) fn history(packed: bool) -> Vec<Message> {
    let mut history = vec![Message {
        role: Role::Assistant,
        content: ["call_a", "call_b"]
            .into_iter()
            .map(|id| ContentPart::ToolCall {
                id: id.into(),
                name: "screenshot".into(),
                arguments: "{}".into(),
                thought_signature: None,
            })
            .collect(),
    }];
    if packed {
        history.push(Message {
            role: Role::Tool,
            content: vec![reply("call_a"), image(), reply("call_b"), image()],
        });
    } else {
        for id in ["call_a", "call_b"] {
            history.push(Message {
                role: Role::Tool,
                content: vec![reply(id), image()],
            });
        }
    }
    history.push(Message {
        role: Role::User,
        content: vec![text("continue")],
    });
    history
}
