#[tokio::test]
async fn collect_stream_completion_updates_tool_name_after_early_delta() {
    let terminal = stream::iter(vec![
        StreamChunk::ToolCallDelta {
            id: "call_4".to_string(),
            arguments_delta: "{\"path\":\"src/provider/openai_codex.rs\"}".to_string(),
        },
        StreamChunk::ToolCallStart {
            id: "call_4".to_string(),
            name: "read".to_string(),
        },
        StreamChunk::Done { usage: None },
    ]);
    let trailing = stream::once(async { panic!("collector polled past terminal Done") });
    let stream = terminal.chain(trailing);

    let response = OpenAiCodexProvider::collect_stream_completion(Box::pin(stream))
        .await
        .expect("stream completion should succeed");

    assert!(matches!(
        response.message.content.first(),
        Some(ContentPart::ToolCall { id, name, arguments, .. })
            if id == "call_4"
                && name == "read"
                && arguments == "{\"path\":\"src/provider/openai_codex.rs\"}"
    ));
}
