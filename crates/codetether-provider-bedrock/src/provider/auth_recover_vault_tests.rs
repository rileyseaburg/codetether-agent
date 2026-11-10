//! Mocked-local regression for live key adoption through the real Vault client.
use super::*;
use std::sync::atomic::Ordering;

#[path = "auth_recover_vault_fixture.rs"]
mod fixture;
#[path = "auth_recover_vault_tests/preflight.rs"]
mod preflight;
#[path = "auth_recover_vault_tests/timeout.rs"]
mod timeout;

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
    preflight::check(&provider, &version, &reads).await;
    timeout::check(&version, &reads).await;
    server.abort();
}
