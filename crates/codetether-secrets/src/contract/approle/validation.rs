//! Reject unsafe endpoint configuration and missing SecretIDs before login.
use super::{AppRoleAuth, Fixture, secret_file};

#[test]
fn configuration_is_endpoint_bound() {
    let file = secret_file();
    for (address, role, mount) in [
        ("http://vault.example", "role", "approle"),
        ("https://user:password@vault.example", "role", "approle"),
        ("https://vault.example", "", "approle"),
        ("https://vault.example", "role", "../other"),
    ] {
        assert!(AppRoleAuth::new(address, role, mount, file.path().into()).is_err());
    }
}

#[tokio::test]
async fn unreadable_secret_id_never_sends_login_request() {
    let server = Fixture::new(vec![]).await;
    let dir = tempfile::tempdir().unwrap();
    let auth = AppRoleAuth::new(
        &server.address,
        "role",
        "approle",
        dir.path().join("missing"),
    )
    .unwrap();
    assert!(auth.login().await.is_err());
    assert!(server.calls.lock().unwrap().requests.is_empty());
}
