//! Mocked-local regression for live key adoption through the real Vault client.
use super::*;
use std::sync::atomic::Ordering;

#[path = "auth_recover_vault_fixture.rs"]
mod fixture;

#[tokio::test]
async fn recovery_adopts_rotated_vault_keys_despite_stale_cache() {
    let (manager, version, reads, server) = fixture::start().await;
    // A cached startup key must not hide later Vault rotations.
    assert_eq!(
        manager.get_api_key("bedrock").await.unwrap().as_deref(),
        Some("fixture-0")
    );
    secrets::init_from_manager(manager).unwrap();
    let provider = BedrockProvider::new("fixture-0".into()).unwrap();
    let active_clone = provider.clone();
    for generation in 1..=2 {
        version.store(generation, Ordering::SeqCst);
        assert!(
            tokio::time::timeout(std::time::Duration::from_secs(5), recover(&provider))
                .await
                .unwrap()
        );
        let expected = format!("fixture-{generation}");
        assert_eq!(
            active_clone.auth.current_bearer().as_deref(),
            Some(expected.as_str())
        );
        // One uncached read per rotation; no second read for an SSO re-mint.
        assert_eq!(reads.load(Ordering::SeqCst), generation + 1);
    }
    // Without a newer key or SSO metadata, recovery stops without replacing auth.
    assert!(
        !tokio::time::timeout(std::time::Duration::from_secs(5), recover(&provider))
            .await
            .unwrap()
    );
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-2"));
    assert_eq!(reads.load(Ordering::SeqCst), 5);
    // Recovery failed earlier; the user now logs in elsewhere and saves a key.
    // Preflight must adopt it before the next request contacts Bedrock.
    version.store(3, Ordering::SeqCst);
    active_clone.prepare_auth().await.unwrap();
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-3"));
    assert_eq!(reads.load(Ordering::SeqCst), 6);
    // An unchanged key must not trigger another SSO refresh.
    provider.prepare_auth().await.unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 7);
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-3"));
    let iam = BedrockProvider::with_credentials(
        crate::provider::AwsCredentials {
            access_key_id: "fixture-id".into(),
            secret_access_key: "fixture-secret".into(),
            session_token: None,
        },
        "us-east-1".into(),
    )
    .unwrap();
    iam.prepare_auth().await.unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 7);

    // A Vault read which never returns must not hang a new completion.
    version.store(usize::MAX, Ordering::SeqCst);
    tokio::time::timeout(std::time::Duration::from_secs(5), provider.prepare_auth())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 8);
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-3"));

    server.abort();
}
