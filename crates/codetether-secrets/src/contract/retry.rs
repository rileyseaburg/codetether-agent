//! retry contract.
use super::{fixture::Fixture, response};
use serde_json::json;

#[tokio::test]
async fn transient_gateway_retry() {
    let server = Fixture::new(vec![
        (503, response::error()),
        (200, response::read(json!({"api_key":"fixture-retry"}))),
    ])
    .await;
    assert!(
        server
            .manager()
            .await
            .get_api_key("provider")
            .await
            .unwrap()
            .as_deref()
            == Some("fixture-retry")
    );
    assert_eq!(server.calls.lock().unwrap().requests.len(), 2);
}
