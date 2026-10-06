use super::acquire;
use super::error::LeaseConflict;
use super::mock_api::lease;
use super::test_support::{conflict, expected, key, repo};
use http::{Method, StatusCode};

#[tokio::test]
async fn denies_active_owner_without_takeover_write() {
    let key = key("Deployment");
    let active = lease(&key.lease_name(), "other", "2999-01-01T00:00:00Z", "7");
    let (repo, requests) = repo(vec![
        expected(Method::POST, StatusCode::CONFLICT, conflict()),
        expected(Method::GET, StatusCode::OK, active),
    ]);
    let error = acquire::acquire(&repo, &key, "mine", 30).await.unwrap_err();
    assert_eq!(
        error.downcast_ref::<LeaseConflict>().unwrap().holder,
        "other"
    );
    assert_eq!(requests.lock().unwrap().len(), 2);
}
