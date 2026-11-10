//! Fresh preflights skip I/O, but a real authentication rejection never does.
use super::*;
use std::sync::atomic::AtomicUsize;

pub(super) async fn check(provider: &BedrockProvider, version: &AtomicUsize, reads: &AtomicUsize) {
    let active_clone = provider.clone();
    version.store(3, Ordering::SeqCst);
    active_clone.prepare_auth().await.unwrap();
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-3"));
    assert_eq!(reads.load(Ordering::SeqCst), 6);
    provider.prepare_auth().await.unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 6);

    version.store(4, Ordering::SeqCst);
    active_clone.prepare_auth().await.unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 6);
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-3"));
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(5), recover(provider))
            .await
            .unwrap()
    );
    assert_eq!(reads.load(Ordering::SeqCst), 7);
    assert_eq!(
        active_clone.auth.current_bearer().as_deref(),
        Some("fixture-4")
    );
}
