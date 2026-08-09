//! Proof that the checkpoint-replay shape was genuinely unpaired *before* the
//! audit ran. Without this, the audit's coverage of the shape is unfalsifiable:
//! a test that passes because the shape was never broken proves nothing.

use super::super::super::convert::convert_messages;
use super::support::call;
use crate::provider::bedrock::body::audit::unpaired;
use crate::provider::{ContentPart, Message, Role};

fn replayed_transcript() -> Vec<Message> {
    vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "go".into() }],
        },
        Message {
            role: Role::Assistant,
            content: vec![call("call_dangling")],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "checkpointed prefix".into(),
            }],
        },
    ]
}

/// `convert_messages` already repairs this shape, so the escape (if any) is not
/// in conversion. Recording the fact pins where the guarantee actually holds.
#[test]
fn conversion_alone_already_pairs_the_checkpoint_replay_shape() {
    let (_, messages) = convert_messages(&replayed_transcript());
    assert!(
        unpaired(&messages).is_none(),
        "conversion left it unpaired: {}",
        serde_json::json!(messages)
    );
}

/// The merge is real: replay collapses into one assistant message, so the
/// toolUse and the replayed text share a turn. This is the structural
/// precondition for the 400, and it is what the audit must survive.
#[test]
fn checkpoint_replay_merges_into_a_single_assistant_turn() {
    let (_, messages) = convert_messages(&replayed_transcript());
    let assistant: Vec<_> = messages
        .iter()
        .filter(|message| message["role"] == "assistant")
        .collect();
    assert_eq!(assistant.len(), 1, "{}", serde_json::json!(messages));
    let content = assistant[0]["content"].as_array().expect("content");
    assert!(
        content
            .iter()
            .any(|part| part.pointer("/toolUse/toolUseId").is_some())
    );
}
