//! Groq streaming preserves reasoning, incremental tool arguments and usage.
use super::*;

#[tokio::test]
async fn groq_stream_retains_reasoning_tools_and_vendor_usage() {
    let payload = format!(
        "{}{}{}{}data: [DONE]\n\n",
        frame(json!({"reasoning":"thinking"})),
        frame(
            json!({"tool_calls":[{"index":0,"id":"call-1","function":{"name":"lookup","arguments":"{\"id\":"}}]})
        ),
        frame(json!({"tool_calls":[{"index":0,"function":{"arguments":"42}"}}]})),
        "data: {\"choices\":[],\"x_groq\":{\"usage\":{\"prompt_tokens\":2,\"completion_tokens\":3,\"total_tokens\":5}}}\n\n"
    );
    let (url, server) = serve(vec![
        groq_limits::catalog("openai/gpt-oss-20b", json!(8192)),
        reply(200, "text/event-stream", payload),
    ]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    let mut input = request("openai/gpt-oss-20b");
    input.stop = vec!["END".into()];
    input.tools.push(ToolDefinition {
        name: "lookup".into(),
        description: "fixture".into(),
        parameters: json!({"type":"object"}),
    });
    let chunks: Vec<_> = provider
        .complete_stream(input)
        .await
        .unwrap()
        .collect()
        .await;
    assert!(matches!(&chunks[0], StreamChunk::Thinking(text) if text == "thinking"));
    assert!(chunks.iter().any(|chunk| matches!(chunk, StreamChunk::ToolCallStart { id, name } if id == "call-1" && name == "lookup")));
    assert!(
        matches!(chunks.last(), Some(StreamChunk::Done { usage: Some(usage) }) if usage.total_tokens == 5)
    );
    let calls = server.join().unwrap();
    let calls = &calls[1..];
    assert_eq!(calls[0].path, "/v1/chat/completions");
    assert_eq!(calls[0].body["model"], "openai/gpt-oss-20b");
    assert_eq!(calls[0].body["max_completion_tokens"], 9);
    assert_eq!(calls[0].body["stop"], json!(["END"]));
    assert_eq!(calls[0].body["stream"], true);
    assert!(calls[0].body.get("top_p").is_some());
    assert!(calls[0].body.get("max_tokens").is_none());
}
