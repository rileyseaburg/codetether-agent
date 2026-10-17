//! Kubernetes token refresh preserves retry and cache invalidation behavior.
use super::{fixture::Fixture, response};
use serde_json::json;
#[tokio::test]
async fn rejected_token_is_refreshed_then_retried() {
    let server = Fixture::new(vec![
        (403, response::error()),
        (200, response::login()),
        (
            200,
            response::read(json!({"api_key":"fixture-refreshed-key"})),
        ),
    ])
    .await;
    let jwt = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(jwt.path(), "fixture-jwt\n").unwrap();
    let mut manager = server.manager().await;
    manager.k8s_auth = Some(std::sync::Arc::new(crate::KubernetesAuthConfig {
        address: server.address.clone(),
        role: "fixture-role".into(),
        auth_mount: "kubernetes".into(),
        jwt_path: jwt.path().to_string_lossy().into_owned(),
    }));
    manager
        .cache
        .write()
        .await
        .insert("old".into(), "fixture-old".into());
    assert!(
        manager.get_api_key("provider").await.unwrap().as_deref() == Some("fixture-refreshed-key")
    );
    assert!(!manager.cache.read().await.contains_key("old"));
    let calls = server.calls.lock().unwrap();
    assert_eq!(calls.requests.len(), 3);
    assert_eq!(calls.requests[1].1, "/v1/auth/kubernetes/login");
    assert!(calls.requests[1].2["jwt"] == "fixture-jwt");
    assert_eq!(calls.requests[1].2["role"], "fixture-role");
}
