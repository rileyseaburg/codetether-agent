//! Denials are bounded and never disclose authentication bodies.
use super::{Fixture, manager, response, secret_file};
use serde_json::json;

#[tokio::test]
async fn persistent_permission_denial_retries_only_once() {
    let server = Fixture::new(vec![
        (200, response::login()),
        (403, response::error()),
        (200, response::login()),
        (403, response::error()),
    ])
    .await;
    let file = secret_file();
    let manager = manager(&server, file.path()).await;
    assert!(manager.read_provider_secrets("monarch").await.is_err());
    assert_eq!(server.calls.lock().unwrap().requests.len(), 4);
}

#[tokio::test]
async fn rejected_identity_does_not_replace_client_or_leak_secrets() {
    let server = Fixture::new(vec![
        (200, response::login()),
        (403, json!({"errors":["fixture-secret fixture-role"]})),
    ])
    .await;
    let file = secret_file();
    let manager = manager(&server, file.path()).await;
    let original = manager.client().unwrap();
    let error = manager
        .refresh_auth(&original)
        .await
        .err()
        .unwrap()
        .to_string();
    assert!(error.contains("AppRole login failed"));
    assert!(!error.contains("fixture-secret"));
    assert!(!error.contains("fixture-role"));
    assert!(std::sync::Arc::ptr_eq(
        &original,
        &manager.client().unwrap()
    ));
    assert_eq!(server.calls.lock().unwrap().requests.len(), 2);
}
