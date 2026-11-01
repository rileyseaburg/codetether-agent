//! Loaded capacity wins; missing metadata must not silently become 128k.

use super::*;

#[tokio::test]
async fn lmstudio_discovery_uses_loaded_capacity_without_cross_provider_leaks() {
    let (url, server) = serve(vec![metadata(4608), metadata(4608)]);
    let models = provider(url.clone()).list_models().await.unwrap();
    assert_eq!(models[0].id, MODEL);
    assert_eq!(models[0].context_window, 4608);
    assert_eq!(models[0].max_output_tokens, Some(1152));
    let other = OpenAIProvider::with_base_url_optional_key(None, url, "other").unwrap();
    assert_eq!(
        other.list_models().await.unwrap()[0].context_window,
        128_000
    );
    let calls = server.join().unwrap();
    assert_eq!(calls[0].path, "/api/v0/models");
    assert_eq!(calls[1].path, "/v1/models");
}

#[tokio::test]
async fn lmstudio_discovery_excludes_unloaded_missing_and_embedding_capacity() {
    let payload = json!({"data": [
        {"id": MODEL, "type": "llm", "state": "loaded", "max_context_length": 262144},
        {"id": "unloaded", "type": "llm", "state": "not-loaded", "loaded_context_length": 4096},
        {"id": "embedding", "type": "embeddings", "state": "loaded", "loaded_context_length": 4096},
        {"id": "zero", "type": "llm", "state": "loaded", "loaded_context_length": 0}
    ]});
    let (url, server) = serve(vec![reply(200, "application/json", payload.to_string())]);
    assert!(provider(url).list_models().await.unwrap().is_empty());
    assert_eq!(server.join().unwrap().len(), 1);
}

#[tokio::test]
async fn lmstudio_discovery_does_not_fallback_on_http_or_json_errors() {
    for response in [
        reply(404, "application/json", "{}".into()),
        reply(200, "application/json", "not json".into()),
        reply(200, "application/json", "{}".into()),
    ] {
        let (url, server) = serve(vec![response]);
        assert!(provider(url).list_models().await.is_err());
        assert_eq!(server.join().unwrap().len(), 1);
    }
}

#[tokio::test]
async fn lmstudio_request_refuses_missing_loaded_capacity_before_chat() {
    let payload = json!({"data": [{"id": MODEL, "type": "llm", "state": "loaded",
        "max_context_length": 262144}]})
    .to_string();
    let (url, server) = serve(vec![reply(200, "application/json", payload)]);
    let error = provider(url).complete(request(MODEL)).await.unwrap_err();
    assert!(error.to_string().contains("no loaded_context_length"));
    assert_eq!(server.join().unwrap().len(), 1);
}

// Capacity is re-read at each request; a model reload cannot leave a cached
// architectural maximum behind. Request tests exercise this separately.
