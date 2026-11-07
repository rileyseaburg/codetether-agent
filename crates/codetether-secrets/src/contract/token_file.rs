//! `VAULT_TOKEN_FILE` startup and rotated-token refresh.
use super::{fixture::Fixture, response};
use crate::token_file::TokenFileAuth;
use serde_json::json;

fn token_file(token: &str) -> tempfile::NamedTempFile {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), format!("{token}\n")).unwrap();
    file
}

fn auth(server: &Fixture, file: &tempfile::NamedTempFile) -> TokenFileAuth {
    TokenFileAuth {
        address: server.address.clone(),
        path: file.path().to_path_buf(),
    }
}

#[tokio::test]
async fn startup_uses_the_file_token() {
    let server = Fixture::new(vec![(
        200,
        response::read(json!({"api_key":"fixture-key"})),
    )])
    .await;
    let file = token_file("s.file-token");
    let manager = crate::SecretsManager::from_token_file(auth(&server, &file), None, None)
        .await
        .unwrap();
    assert_eq!(
        manager.get_api_key("provider").await.unwrap().as_deref(),
        Some("fixture-key")
    );
    assert_eq!(server.calls.lock().unwrap().tokens, ["s.file-token"]);
}

#[tokio::test]
async fn empty_token_file_fails_startup() {
    let server = Fixture::new(vec![]).await;
    let file = token_file("  ");
    let error = crate::SecretsManager::from_token_file(auth(&server, &file), None, None)
        .await
        .err()
        .expect("empty token file must fail")
        .to_string();
    assert!(error.contains("is empty"), "{error}");
}

#[tokio::test]
async fn rejected_token_rereads_the_rotated_file() {
    let server = Fixture::new(vec![
        (403, response::error()),
        (
            200,
            response::read(json!({"api_key":"fixture-rotated-key"})),
        ),
    ])
    .await;
    let file = token_file("s.stale-token");
    let manager = crate::SecretsManager::from_token_file(auth(&server, &file), None, None)
        .await
        .unwrap();
    manager
        .cache
        .write()
        .await
        .insert("old".into(), "fixture-old".into());
    std::fs::write(file.path(), "s.rotated-token\n").unwrap();
    assert_eq!(
        manager.get_api_key("provider").await.unwrap().as_deref(),
        Some("fixture-rotated-key")
    );
    assert!(!manager.cache.read().await.contains_key("old"));
    let calls = server.calls.lock().unwrap();
    assert_eq!(calls.tokens, ["s.stale-token", "s.rotated-token"]);
}

#[tokio::test]
async fn second_rejection_after_reload_is_not_retried_again() {
    let server = Fixture::new(vec![(403, response::error()), (403, response::error())]).await;
    let file = token_file("s.revoked-token");
    let manager = crate::SecretsManager::from_token_file(auth(&server, &file), None, None)
        .await
        .unwrap();
    assert!(manager.get_api_key("provider").await.unwrap().is_none());
    assert_eq!(server.calls.lock().unwrap().requests.len(), 2);
}
