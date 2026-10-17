//! failures contract.
use super::{fixture::Fixture, response};
use serde_json::json;

#[tokio::test]
async fn missing_and_denied_reads() {
    let server = Fixture::new(vec![
        (404, response::error()),
        (403, response::error()),
        (403, response::error()),
        (404, response::error()),
        (404, response::error()),
    ])
    .await;
    let manager = server.manager().await;
    assert!(manager.get_api_key("missing").await.unwrap().is_none());
    assert!(
        manager
            .get_provider_secrets("denied")
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        manager
            .list_configured_providers()
            .await
            .unwrap()
            .is_empty()
    );
    assert!(manager.verify_reachable().await.is_ok());
    assert_eq!(server.calls.lock().unwrap().requests.len(), 5);
}
