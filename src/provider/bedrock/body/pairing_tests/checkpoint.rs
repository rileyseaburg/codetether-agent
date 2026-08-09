//! SRP checkpoint replay shape: a retry appends a second assistant message,
//! which `convert::append_assistant` merges into the preceding assistant turn.
//! If that earlier turn held a dangling toolUse, the merged message carries an
//! unanswered toolUse — the shape Bedrock rejects with a 400.

use super::support::{call, request};
use crate::provider::bedrock::body::audit::unpaired;
use crate::provider::bedrock::build_converse_body;
use crate::provider::{ContentPart, Message, Role};

#[test]
fn checkpoint_replay_after_a_dangling_call_stays_paired() {
    let body = build_converse_body(
        &request(vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: "go".into() }],
            },
            Message {
                role: Role::Assistant,
                content: vec![call("call_dangling")],
            },
            // SRP checkpoint replay pushes retained content as its own
            // assistant message; conversion merges it with the turn above.
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: "checkpointed prefix".into(),
                }],
            },
        ]),
        "us.anthropic.claude-opus-4-7",
    );
    let messages = body["messages"].as_array().expect("messages array");
    assert!(
        unpaired(messages).is_none(),
        "checkpoint replay left an unpaired toolUse: {body}"
    );
}
