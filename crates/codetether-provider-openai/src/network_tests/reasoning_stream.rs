//! Raw reasoning streams retain aliases, reasoning, usage termination, and optional auth.

use super::*;
#[tokio::test]
async fn reasoning_stream_retains_alias_and_optional_authentication() {
    let payload = format!(
        "{}{}data: [DONE]\n\n",
        frame(json!({"reasoning_content":"think"})),
        frame(json!({"content":"hello"}))
    );
    let (url, server) = serve(vec![reply(200, "text/event-stream", payload)]);
    let provider = OpenAIProvider::with_base_url_optional_key(None, url, "cerebras").unwrap();
    let chunks: Vec<_> = provider
        .complete_stream(request("glm-4.7"))
        .await
        .unwrap()
        .collect()
        .await;
    assert!(matches!(&chunks[0], StreamChunk::Thinking(text) if text == "think"));
    assert!(matches!(&chunks[1], StreamChunk::Text(text) if text == "hello"));
    assert!(matches!(&chunks[2], StreamChunk::Done { .. }));
    let calls = server.join().unwrap();
    assert_eq!(calls[0].body["model"], "zai-glm-4.7");
    assert!(calls[0].body.get("top_p").is_some());
    assert!(
        !calls[0]
            .headers
            .to_ascii_lowercase()
            .contains("authorization:")
    );
}
