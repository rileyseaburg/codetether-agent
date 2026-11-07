//! Each Groq model uses its own advertised cap, not a Qwen-specific constant.
use super::*;

#[tokio::test]
async fn groq_uses_each_models_output_limit_for_both_transports() {
    // Deliberately distinct fixture limits; these are not live catalog claims.
    let models = [
        ("qwen/qwen3.8-27b", 16384),
        ("openai/gpt-oss-120b", 32768),
        ("llama-3.3-70b-versatile", 8192),
    ];
    let entries: Vec<_> = models
        .iter()
        .map(|(id, max)| {
            json!({
                "id":id, "context_window":131072, "max_completion_tokens":max
            })
        })
        .collect();
    let mut replies = vec![reply(
        200,
        "application/json",
        json!({"data":entries}).to_string(),
    )];
    for _ in models {
        replies.push(reply(200, "application/json", chat()));
        replies.push(reply(
            200,
            "text/event-stream",
            format!("{}data: [DONE]\n\n", frame(json!({"content":"ok"}))),
        ));
    }
    let (url, server) = serve(replies);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    for (model, _) in models {
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
        assert!(matches!(chunks.last(), Some(StreamChunk::Done { .. })));
    }
    let calls = server.join().unwrap();
    assert_eq!(calls[0].path, "/v1/models");
    for (calls, (model, limit)) in calls[1..].chunks_exact(2).zip(models) {
        for call in calls {
            assert_eq!(call.body["model"], model);
            assert_eq!(call.body["max_completion_tokens"], limit);
        }
    }
}
