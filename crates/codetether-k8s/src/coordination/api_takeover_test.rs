use super::acquire;
use super::mock_api::lease;
use super::test_support::{conflict, expected, key, repo};
use http::{Method, StatusCode};

#[tokio::test]
async fn expired_owner_is_replaced_with_resource_version() {
    let key = key("Pod");
    let old = lease(&key.lease_name(), "other", "2020-01-01T00:00:00Z", "7");
    let new = lease(&key.lease_name(), "mine", "2026-01-01T00:00:00Z", "8");
    let (repo, requests) = repo(vec![
        expected(Method::POST, StatusCode::CONFLICT, conflict()),
        expected(Method::GET, StatusCode::OK, old),
        expected(Method::PUT, StatusCode::OK, new),
    ]);
    acquire::acquire(&repo, &key, "mine", 30).await.unwrap();
    let request = requests.lock().unwrap().pop().unwrap();
    let bytes = request.into_body().collect_bytes().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        body.pointer("/metadata/resourceVersion")
            .and_then(|v| v.as_str()),
        Some("7")
    );
}
