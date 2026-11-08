//! Completion and streaming must share the same fresh capacity guard.

use super::*;

#[tokio::test]
async fn lmstudio_caps_output_for_completion_and_streaming() {
    let stream = format!(
        "{}{}data: [DONE]\n\n",
        frame(json!({"reasoning_content":"think"})),
        frame(json!({"content":"hello"})),
    );
    let (url, server) = serve(vec![
        metadata(4608),
        reply(200, "application/json", chat()),
        metadata(4608),
        reply(200, "text/event-stream", stream),
    ]);
    let provider = provider(url);
    let mut input = request(MODEL);
    input.max_tokens = Some(8192);
    provider.complete(input.clone()).await.unwrap();
    input.max_tokens = None;
    let chunks: Vec<_> = provider
        .complete_stream(input)
        .await
        .unwrap()
        .collect()
        .await;
    assert!(matches!(&chunks[0], StreamChunk::Thinking(text) if text == "think"));
    assert!(matches!(&chunks[1], StreamChunk::Text(text) if text == "hello"));
    assert!(matches!(chunks.last(), Some(StreamChunk::Done { .. })));
    let calls = server.join().unwrap();
    assert_eq!(calls[0].path, "/api/v0/models");
    assert_eq!(calls[1].body["max_tokens"], 1152);
    assert_eq!(calls[2].path, "/api/v0/models");
    assert_eq!(calls[3].body["max_tokens"], 1152);
}

#[tokio::test]
async fn lmstudio_rechecks_capacity_after_reload_and_rejects_before_post() {
    let (url, server) = serve(vec![metadata(4608), metadata(256)]);
    let provider = provider(url);
    assert_eq!(
        provider.list_models().await.unwrap()[0].context_window,
        4608
    );
    let error = provider.complete(request(MODEL)).await.unwrap_err();
    assert!(error.to_string().contains("loaded capacity 256"));
    let calls = server.join().unwrap();
    assert!(calls.iter().all(|call| call.path == "/api/v0/models"));
}

#[tokio::test]
async fn lmstudio_caps_output_to_remaining_space_without_dropping_input() {
    let (url, server) = serve(vec![metadata(4608), reply(200, "application/json", chat())]);
    let mut input = request(MODEL);
    let text = "x".repeat(8500);
    input.messages[0].content = vec![ContentPart::Text { text: text.clone() }];
    input.max_tokens = Some(8192);
    provider(url).complete(input).await.unwrap();
    let calls = server.join().unwrap();
    let body = &calls[1].body;
    let reserved = serde_json::to_vec(&json!({"messages": body["messages"], "tools": []}))
        .unwrap()
        .len()
        .div_ceil(3)
        + 256
        + 32;
    let output = body["max_tokens"].as_u64().unwrap() as usize;
    assert_eq!(output, 4608 * 4 / 5 - reserved);
    assert!(output < 1024);
    assert_eq!(body["messages"][0]["content"], text);
}
