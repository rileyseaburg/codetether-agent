//! Embeddings preserve ordering, usage fallback, empty batches, and length checks.

use super::*;
#[tokio::test]
async fn embedding_responses_are_ordered_and_validated() {
    let body = json!({"data":[{"index":1,"embedding":[2.0]},{"index":0,"embedding":[1.0]}],"usage":{"prompt_tokens":4}}).to_string();
    let invalid = json!({"data":[{"index":0,"embedding":[1.0]}]}).to_string();
    let (url, server) = serve(vec![
        reply(200, "application/json", body),
        reply(200, "application/json", invalid),
    ]);
    let provider = OpenAIProvider::with_base_url_optional_key(None, url, "custom").unwrap();
    let empty = provider
        .embed(EmbeddingRequest {
            model: "model".into(),
            inputs: Vec::new(),
        })
        .await
        .unwrap();
    assert!(empty.embeddings.is_empty());
    let input = || EmbeddingRequest {
        model: "model".into(),
        inputs: vec!["a".into(), "b".into()],
    };
    let response = provider.embed(input()).await.unwrap();
    assert_eq!(response.embeddings, vec![vec![1.0], vec![2.0]]);
    assert_eq!(response.usage.total_tokens, 4);
    assert!(
        provider
            .embed(input())
            .await
            .unwrap_err()
            .to_string()
            .contains("length mismatch")
    );
    let calls = server.join().unwrap();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].path, "/v1/embeddings");
    assert!(
        !calls[0]
            .headers
            .to_ascii_lowercase()
            .contains("authorization:")
    );
}
