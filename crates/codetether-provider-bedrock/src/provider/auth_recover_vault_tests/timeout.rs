//! IAM skips Vault; a stalled Vault preserves credentials and gets a retry window.
use super::*;
use std::sync::atomic::AtomicUsize;

pub(super) async fn check(version: &AtomicUsize, reads: &AtomicUsize) {
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

    let provider = BedrockProvider::new("fixture-4".into()).unwrap();
    version.store(usize::MAX, Ordering::SeqCst);
    tokio::time::timeout(std::time::Duration::from_secs(5), provider.prepare_auth())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 8);
    assert_eq!(provider.auth.current_bearer().as_deref(), Some("fixture-4"));
    // The failed/slow optional check is throttled too, not retried on every call.
    tokio::time::timeout(std::time::Duration::from_secs(5), provider.prepare_auth())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 8);
}
