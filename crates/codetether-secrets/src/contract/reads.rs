//! reads contract.
use super::{fixture::Fixture, response};
use serde_json::json;

#[tokio::test]
async fn cache_and_provider_reads() {
    let server = Fixture::new(vec![
        (
            200,
            response::read(json!({"api_key":"fixture-first","custom":7})),
        ),
        (200, response::read(json!({"api_key":"fixture-second"}))),
        (
            200,
            response::read(json!({"base_url":"http://fixture","custom":9})),
        ),
    ])
    .await;
    let manager = server.manager().await;
    assert!(manager.get_api_key("provider").await.unwrap().as_deref() == Some("fixture-first"));
    assert!(manager.get_api_key("provider").await.unwrap().as_deref() == Some("fixture-first"));
    manager.clear_cache().await;
    assert!(manager.get_api_key("provider").await.unwrap().as_deref() == Some("fixture-second"));
    let record = manager
        .get_provider_secrets("provider")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(record.extra["custom"], 9);
    assert_eq!(record.base_url.as_deref(), Some("http://fixture"));
    let calls = server.calls.lock().unwrap();
    assert_eq!(calls.requests.len(), 3);
    assert!(
        calls
            .requests
            .iter()
            .all(|(_, path, _)| path == "/v1/secret/data/codetether/providers/provider")
    );
}
