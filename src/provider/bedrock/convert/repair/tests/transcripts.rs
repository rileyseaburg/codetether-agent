//! Transcript shapes seen in real agent loops, checked against the pairing
//! invariant Bedrock enforces server-side.

use super::invariant::violation;
use crate::provider::bedrock::convert::convert_messages;
use crate::provider::{ContentPart, Message, Role};

#[path = "transcripts/shapes.rs"]
mod shapes;

fn assistant(parts: Vec<ContentPart>) -> Message {
    Message {
        role: Role::Assistant,
        content: parts,
    }
}

fn tool(parts: Vec<ContentPart>) -> Message {
    Message {
        role: Role::Tool,
        content: parts,
    }
}

fn assert_pairs(input: &[Message], label: &str) {
    let (_, messages) = convert_messages(input);
    if let Some(problem) = violation(&messages) {
        panic!("{label}: {problem}\n{}", serde_json::json!(messages));
    }
}

#[test]
fn interleaved_assistant_text_then_tool_call_pairs() {
    assert_pairs(&shapes::interleaved(), "interleaved");
}

#[test]
fn assistant_narration_between_two_tool_rounds_pairs() {
    assert_pairs(&shapes::narrated(), "narration between rounds");
}

#[test]
fn tool_result_arriving_before_its_call_does_not_orphan() {
    assert_pairs(&shapes::stale_first(), "stale-first");
}
