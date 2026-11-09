//! Reproduce observed call shapes through Rust validation and streamed events.
use super::{fixture, stream};
use codetether_gemini_web_protocol::{prompt, stream_output, tool_validation};
use codetether_provider_core::{ContentPart, Message, Role, StreamChunk, Usage};
use futures::StreamExt;

#[tokio::test]
async fn native_protocol_preserves_sequential_tool_result_feedback() {
    let data = fixture::Fixture::new();
    let tools = fixture::tools();
    let mut messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: fixture::TASK.into(),
        }],
    }];
    let markup = [
        r#"<tool_call>{"name":"lookup_probe","arguments":{"id":"probe-native"}}</tool_call>"#
            .to_owned(),
        format!(
            r#"<tool_call>{{"name":"multiply","arguments":{{"a":{},"b":7}}}}</tool_call>"#,
            data.quantity
        ),
    ];
    assert!(
        !prompt::render(&messages, &tools)
            .unwrap()
            .contains(&data.code)
    );
    for (step, text) in markup.iter().enumerate() {
        let (cleaned, calls) = tool_validation::extract(text, &tools, &messages).unwrap();
        let chunks = stream_output::from_parts(cleaned, calls, Usage::default());
        let message = stream::collect(futures::stream::iter(chunks).boxed())
            .await
            .unwrap();
        let [
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            },
        ] = message.content.as_slice()
        else {
            panic!("expected one structured call, not raw XML text");
        };
        let content = data.execute(step, name, arguments).unwrap().to_string();
        let result = Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.clone(),
                content,
            }],
        };
        messages.extend([message, result]);
        let followup = prompt::render(&messages, &tools).unwrap();
        assert!(followup.contains("Tool: <tool_result>"));
        assert!(followup.contains(&data.code));
    }
    let (text, calls) = tool_validation::extract(&data.expected(), &tools, &messages).unwrap();
    assert!(calls.is_empty());
    assert_eq!(text, data.expected());
}

#[tokio::test]
async fn unfinished_or_mismatched_streamed_calls_are_rejected() {
    let chunks = vec![
        StreamChunk::ToolCallStart {
            id: "a".into(),
            name: "multiply".into(),
        },
        StreamChunk::ToolCallDelta {
            id: "b".into(),
            arguments_delta: "{}".into(),
        },
    ];
    assert!(
        stream::collect(futures::stream::iter(chunks).boxed())
            .await
            .is_err()
    );
    assert!(
        stream::collect(futures::stream::empty().boxed())
            .await
            .is_err()
    );
    assert!(data_rejects_unadvertised_tool());
}
fn data_rejects_unadvertised_tool() -> bool {
    fixture::Fixture::new()
        .execute(0, "exec_command", r#"{"cmd":"no"}"#)
        .is_err()
}
