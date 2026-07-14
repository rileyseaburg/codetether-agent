use super::mock_api::lease;
use super::renew;
use super::test_support::{expected, key, repo};
use http::{Method, StatusCode};

#[tokio::test]
async fn stale_owner_does_not_issue_delete() {
    let key = key("Pod");
    let successor = lease(&key.lease_name(), "successor", "2999-01-01T00:00:00Z", "9");
    let (repo, requests) = repo(vec![expected(Method::GET, StatusCode::OK, successor)]);
    let released = renew::release(&repo, &key.lease_name(), "stale")
        .await
        .unwrap();
    assert!(!released);
    assert_eq!(requests.lock().unwrap().len(), 1);
}
