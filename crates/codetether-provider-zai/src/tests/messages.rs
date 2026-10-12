//! Historical assistant tool-call encoding regression tests.
use crate::{ContentPart, Message, Role};
use serde_json::{Value, json};

#[test]
fn historical_arguments_are_json_strings() {
    for (raw, expected) in [
        (r#"{"city":"Beijing".. }"#, json!({"city":"Beijing"})),
        ("city=Beijing", json!({"input":"city=Beijing"})),
        (r#""Beijing""#, json!({"input":"Beijing"})),
        (r#"{"city":"Beijing"}"#, json!({"city":"Beijing"})),
    ] {
        let message = Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".into(),
                name: "weather".into(),
                arguments: raw.into(),
                thought_signature: None,
            }],
        };
        let wire = crate::messages::convert(&[message], false);
        let function = &wire[0]["tool_calls"][0]["function"];
        let encoded = function["arguments"].as_str().unwrap();
        assert_eq!(serde_json::from_str::<Value>(encoded).unwrap(), expected);
        assert_eq!(wire[0]["content"], Value::Null);
    }
}

#[test]
fn assistant_tool_calls_preserve_text_but_omit_reasoning() {
    let message = Message {
        role: Role::Assistant,
        content: vec![
            ContentPart::Text {
                text: "Using a tool.".into(),
            },
            ContentPart::Thinking {
                text: "private reasoning".into(),
                signature: None,
            },
            ContentPart::ToolCall {
                id: "call_1".into(),
                name: "weather".into(),
                arguments: "{}".into(),
                thought_signature: None,
            },
        ],
    };
    let wire = crate::messages::convert(&[message], false);
    assert_eq!(wire[0]["content"], "Using a tool.");
    assert!(wire[0].get("reasoning_content").is_none());
}
