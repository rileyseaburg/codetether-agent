//! Login, rotated SecretID recovery, and clone coordination.
use super::{Fixture, manager, response, secret_file};
use serde_json::json;

#[tokio::test]
async fn rejection_reauthenticates_with_rotated_secret_id_and_retries() {
    let server = Fixture::new(vec![
        (200, response::login()),
        (403, response::error()),
        (200, response::login()),
        (200, response::read(json!({"api_key":"fixture-key"}))),
    ])
    .await;
    let file = secret_file();
    let manager = manager(&server, file.path()).await;
    manager
        .cache
        .write()
        .await
        .insert("old".into(), "fixture-old".into());
    std::fs::write(file.path(), "fixture-rotated\n").unwrap();
    let record = manager
        .read_provider_secrets("monarch")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(record.api_key.as_deref(), Some("fixture-key"));
    assert!(manager.cache.read().await.is_empty());
    let calls = server.calls.lock().unwrap();
    assert_eq!(calls.requests.len(), 4);
    assert_eq!(calls.requests[0].1, "/v1/auth/approle/login");
    assert_eq!(calls.requests[0].2["secret_id"], "fixture-secret");
    assert_eq!(calls.requests[2].2["secret_id"], "fixture-rotated");
    assert_eq!(calls.requests[2].2["role_id"], "fixture-role");
}

#[tokio::test]
async fn clones_share_one_reauthentication_for_the_rejected_client() {
    let server = Fixture::new(vec![(200, response::login()), (200, response::login())]).await;
    let file = secret_file();
    let manager = manager(&server, file.path()).await;
    let clone = manager.clone();
    let rejected = manager.client().unwrap();
    let (a, b) = tokio::join!(
        manager.refresh_auth(&rejected),
        clone.refresh_auth(&rejected)
    );
    let a = a.unwrap().unwrap();
    let b = b.unwrap().unwrap();
    assert!(std::sync::Arc::ptr_eq(&a, &b));
    assert!(!std::sync::Arc::ptr_eq(&a, &rejected));
    assert_eq!(server.calls.lock().unwrap().requests.len(), 2);
}
