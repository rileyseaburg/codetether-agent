//! Model discovery uses bounded JSON reads and the existing empty fallback.

use super::*;
#[tokio::test]
async fn model_discovery_accepts_json_and_rejects_oversized_headers() {
    let payload = json!({"data":[{"id":"fixture-model"}]}).to_string();
    let oversized = Reply {
        status: 200,
        content_type: "application/json",
        body: String::new(),
        declared_len: Some(9 * 1024 * 1024),
    };
    let (url, server) = serve(vec![reply(200, "application/json", payload), oversized]);
    let provider = OpenAIProvider::with_base_url_optional_key(None, url, "custom").unwrap();
    let models = provider.list_models().await.unwrap();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "fixture-model");
    assert!(provider.list_models().await.unwrap().is_empty());
    let calls = server.join().unwrap();
    assert_eq!(calls[0].path, "/v1/models");
    assert_eq!(calls[1].path, "/v1/models");
}
