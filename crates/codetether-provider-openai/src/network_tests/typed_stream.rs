//! SDK streaming keeps typed request options and delta conversion.

use super::*;
#[tokio::test]
async fn typed_stream_preserves_request_options_and_text() {
    let payload = format!("{}data: [DONE]\n\n", frame(json!({"content":"hello"})));
    let (url, server) = serve(vec![reply(200, "text/event-stream", payload)]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "openai").unwrap();
    let chunks: Vec<_> = provider
        .complete_stream(request("model"))
        .await
        .unwrap()
        .collect()
        .await;
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "hello"));
    let calls = server.join().unwrap();
    assert_eq!(calls[0].body["max_completion_tokens"], 9);
    assert_eq!(calls[0].body["stream"], true);
    assert!(calls[0].body.get("top_p").is_none());
}
