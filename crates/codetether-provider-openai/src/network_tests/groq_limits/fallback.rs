//! Missing metadata must not invent a cap or poison subsequent discovery.
use super::*;

#[tokio::test]
async fn groq_retries_discovery_after_failure_without_blocking_inference() {
    let (url, server) = serve(vec![
        reply(503, "application/json", "{}".into()),
        reply(200, "application/json", chat()),
        catalog("fixture", json!(16384)),
        reply(200, "application/json", chat()),
    ]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    let mut input = request("fixture");
    input.max_tokens = Some(64000);
    provider.complete(input.clone()).await.unwrap();
    provider.complete(input).await.unwrap();
    let calls = server.join().unwrap();
    assert_eq!(calls[1].body["max_completion_tokens"], 64000);
    assert_eq!(calls[3].body["max_completion_tokens"], 16384);
}

#[tokio::test]
async fn groq_preserves_smaller_equal_unknown_and_unspecified_limits() {
    let (url, server) = serve(vec![
        catalog("fixture", json!(16384)),
        reply(200, "application/json", chat()),
        reply(200, "application/json", chat()),
        reply(200, "application/json", chat()),
        reply(200, "application/json", chat()),
    ]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    provider.list_models().await.unwrap();
    for (model, max) in [
        ("fixture", Some(8192)),
        ("fixture", Some(16384)),
        ("unknown", Some(64000)),
        ("fixture", None),
    ] {
        let mut input = request(model);
        input.max_tokens = max;
        provider.complete(input).await.unwrap();
    }
    let calls = server.join().unwrap();
    assert_eq!(calls[1].body["max_completion_tokens"], 8192);
    assert_eq!(calls[2].body["max_completion_tokens"], 16384);
    assert_eq!(calls[3].body["max_completion_tokens"], 64000);
    assert!(calls[4].body.get("max_completion_tokens").is_none());
}
