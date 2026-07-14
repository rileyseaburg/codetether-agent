use super::ensure;
use crate::provider::{ContentPart, Message, Role};
use serde_json::json;

#[test]
fn inserts_anchor_when_converted_input_is_empty() {
    let input = ensure(Vec::new());
    assert_eq!(input.len(), 1);
    assert_eq!(input[0]["role"], "user");
    assert!(!input[0]["content"][0]["text"].as_str().unwrap().is_empty());
}

#[test]
fn system_only_context_gets_a_responses_anchor() {
    let messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: "instructions".into(),
        }],
    }];
    let input =
        super::super::super::OpenAiCodexProvider::convert_messages_to_responses_input(&messages);
    assert_eq!(input[0]["role"], "user");
}

#[test]
fn preserves_existing_input() {
    let original = vec![json!({"type": "message", "role": "user"})];
    assert_eq!(ensure(original.clone()), original);
}
