//! Mocked Groq catalog and output-limit regressions through both transports.
use super::*;

pub(super) fn catalog(model: &str, limit: Value) -> Reply {
    reply(
        200,
        "application/json",
        json!({"data":[{
            "id":model, "context_window":131072, "max_completion_tokens":limit
        }]})
        .to_string(),
    )
}

#[tokio::test]
async fn groq_caps_qwen_output_for_completion_and_stream_and_reuses_discovery() {
    let model = "qwen/qwen3.8-27b";
    let (url, server) = serve(vec![
        catalog(model, json!(16384)),
        reply(200, "application/json", chat()),
        reply(
            200,
            "text/event-stream",
            format!("{}data: [DONE]\n\n", frame(json!({"content":"ok"}))),
        ),
    ]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    let mut input = request(model);
    input.max_tokens = Some(64000);
    provider.complete(input.clone()).await.unwrap();
    let chunks: Vec<_> = provider
        .complete_stream(input)
        .await
        .unwrap()
        .collect()
        .await;
    assert!(
        chunks
            .iter()
            .all(|chunk| !matches!(chunk, StreamChunk::Error(_)))
    );
    assert_eq!(
        provider.list_models().await.unwrap()[0].max_output_tokens,
        Some(16384)
    );
    let calls = server.join().unwrap();
    assert_eq!(calls[0].path, "/v1/models");
    for call in &calls[1..] {
        assert_eq!(call.path, "/v1/chat/completions");
        assert_eq!(call.body["model"], model);
        assert_eq!(call.body["max_completion_tokens"], 16384);
    }
}

#[tokio::test]
async fn groq_preserves_requested_output_when_limit_is_missing_or_zero() {
    for limit in [Value::Null, json!(0)] {
        let (url, server) = serve(vec![
            catalog("fixture", limit),
            reply(200, "application/json", chat()),
        ]);
        let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
        let mut input = request("fixture");
        input.max_tokens = Some(64000);
        provider.complete(input).await.unwrap();
        let calls = server.join().unwrap();
        assert_eq!(calls[1].body["max_completion_tokens"], 64000);
    }
}

mod fallback;
mod multiple_models;
