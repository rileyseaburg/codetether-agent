use super::acquire;
use super::mock_api::lease;
use super::test_support::{expected, key, repo};
use http::{Method, StatusCode};

#[tokio::test]
async fn creates_lease_for_unowned_resource() {
    let key = key("Deployment");
    let body = lease(&key.lease_name(), "mine", "2026-01-01T00:00:00Z", "1");
    let (repo, requests) = repo(vec![expected(Method::POST, StatusCode::CREATED, body)]);
    let acquired = acquire::acquire(&repo, &key, "mine", 30).await.unwrap();
    assert_eq!(
        acquired.spec.unwrap().holder_identity.as_deref(),
        Some("mine")
    );
    assert_eq!(
        requests.lock().unwrap()[0].uri().path(),
        "/apis/coordination.k8s.io/v1/namespaces/default/leases"
    );
}
