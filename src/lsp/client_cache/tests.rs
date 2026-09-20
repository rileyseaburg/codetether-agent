//! Idle eviction must never reclaim a client held by an active operation.
use super::eviction::expired;
use std::time::Duration;

#[cfg(unix)]
#[tokio::test]
async fn idle_cache_evicts_released_clients_but_preserves_borrowers() {
    use super::{Entries, LspClient, eviction::evict};
    use crate::lsp::types::LspConfig;
    use std::sync::Arc;
    use std::time::Instant;

    let config = LspConfig {
        command: "/bin/sh".into(),
        args: vec!["-c".into(), "sleep 60".into()],
        ..Default::default()
    };
    let client = Arc::new(LspClient::new(config).await.unwrap());
    let mut entries = Entries::new();
    entries.insert("test".into(), Arc::clone(&client));
    *client.last_used.lock().unwrap() = Instant::now() - Duration::from_secs(121);
    evict(&mut entries);
    assert_eq!(entries.len(), 1, "active borrower must survive");
    *client.last_used.lock().unwrap() = Instant::now() - Duration::from_secs(121);
    let weak = Arc::downgrade(&client);
    drop(client);
    evict(&mut entries);
    assert!(entries.is_empty());
    assert!(weak.upgrade().is_none());
}

#[test]
fn idle_cache_releases_only_expired_unborrowed_clients() {
    assert!(!expired(1, Duration::from_secs(119)));
    assert!(expired(1, Duration::from_secs(120)));
    assert!(!expired(2, Duration::from_secs(600)));
}

#[tokio::test]
async fn idle_cache_concurrent_access_uses_one_cache() {
    let cache = super::ClientCache::default();
    let guard = cache.write().await;
    assert!(
        tokio::time::timeout(Duration::from_millis(10), cache.write())
            .await
            .is_err()
    );
    drop(guard);
    assert!(cache.write().await.is_empty());
}
