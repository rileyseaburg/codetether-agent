//! The final Converse body must never contain an unpaired toolUse, including
//! after prompt-cache breakpoints are appended.

use crate::provider::bedrock::body::audit::unpaired;
use crate::provider::bedrock::build_converse_body;
use crate::provider::{ContentPart, Message, Role};

#[path = "pairing_tests/support.rs"]
mod support;

#[path = "pairing_tests/checkpoint.rs"]
mod checkpoint;
#[path = "pairing_tests/checkpoint_shape.rs"]
mod checkpoint_shape;

use support::{call, request};

#[test]
fn dangling_tool_call_never_reaches_the_wire_unpaired() {
    let body = build_converse_body(
        &request(vec![Message {
            role: Role::Assistant,
            content: vec![call("call_70YI3UTXkzXNIcpQwDZQMvD8")],
        }]),
        "us.anthropic.claude-opus-4-7",
    );
    let messages = body["messages"].as_array().expect("messages array");
    assert!(
        unpaired(messages).is_none(),
        "unpaired toolUse survived body build: {body}"
    );
}

#[test]
fn cache_point_after_a_tool_call_does_not_orphan_it() {
    let body = build_converse_body(
        &request(vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: "go".into() }],
            },
            Message {
                role: Role::Assistant,
                content: vec![call("call_cache")],
            },
        ]),
        "us.anthropic.claude-opus-4-7",
    );
    let messages = body["messages"].as_array().expect("messages array");
    assert!(unpaired(messages).is_none(), "{body}");
}
