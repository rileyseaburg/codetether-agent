fn shared_refresh_credentials(refresh: &str) -> OAuthCredentials {
    OAuthCredentials {
        id_token: None,
        chatgpt_account_id: None,
        access_token: format!("access-for-{refresh}"),
        refresh_token: refresh.into(),
        expires_at: 100,
    }
}

#[test]
fn shared_credentials_adopted_only_after_peer_rotation() {
    let mut local = shared_refresh_credentials("r1");
    local.chatgpt_account_id = Some("account".into());

    assert!(adopt_shared_credentials(&local, shared_refresh_credentials("r1")).is_none());

    let adopted = adopt_shared_credentials(&local, shared_refresh_credentials("r2"))
        .expect("peer rotation should be adopted");
    assert_eq!(adopted.refresh_token, "r2");
    assert_eq!(adopted.access_token, "access-for-r2");
    assert_eq!(adopted.chatgpt_account_id.as_deref(), Some("account"));
}

#[tokio::test]
async fn refresh_lock_excludes_second_holder_until_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("locks").join("codex.lock");
    let short = std::time::Duration::from_millis(250);

    let first = RefreshLock::acquire_at(&path, short).await.unwrap();
    assert!(RefreshLock::acquire_at(&path, short).await.is_err());

    drop(first);
    assert!(RefreshLock::acquire_at(&path, short).await.is_ok());
}
