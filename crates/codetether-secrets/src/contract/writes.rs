//! Provider writes update or remove the cached API key.
use super::{fixture::Fixture, response};
use serde_json::json;
#[tokio::test]
async fn writing_and_listing_providers() {
    let server = Fixture::new(vec![
        (200, json!({"data":response::metadata()})),
        (200, json!({"data":response::metadata()})),
        (200, json!({"data":{"keys":["one","two"]}})),
    ])
    .await;
    let manager = server.manager().await;
    let secrets = crate::ProviderSecrets {
        api_key: Some("fixture-write".into()),
        ..Default::default()
    };
    manager
        .set_provider_secrets("provider", &secrets)
        .await
        .unwrap();
    assert!(manager.get_api_key("provider").await.unwrap().as_deref() == Some("fixture-write"));
    manager
        .set_provider_secrets("provider", &crate::ProviderSecrets::default())
        .await
        .unwrap();
    assert!(!manager.cache.read().await.contains_key("provider"));
    assert_eq!(
        manager.list_configured_providers().await.unwrap(),
        ["one", "two"]
    );
    let calls = server.calls.lock().unwrap();
    assert_eq!(calls.requests.len(), 3);
    assert!(calls.requests[0].2["data"]["api_key"] == "fixture-write");
}
