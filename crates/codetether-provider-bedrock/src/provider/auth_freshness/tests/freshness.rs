//! Freshness expiry and instance isolation do not depend on wall-clock sleeps.
use super::*;
use crate::provider::BedrockProvider;

#[tokio::test(start_paused = true)]
async fn fresh_attempts_skip_until_the_exact_ttl_boundary() {
    let gate = AuthFreshness::default();
    let reads = AtomicUsize::new(0);
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
    advance(TTL - Duration::from_millis(1)).await;
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
    advance(Duration::from_millis(1)).await;
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 2);
}

#[tokio::test(start_paused = true)]
async fn provider_clones_share_checks_but_credential_instances_do_not() {
    let first = BedrockProvider::new("fixture-first".into()).unwrap();
    let cloned = first.clone();
    let separate = BedrockProvider::new("fixture-second".into()).unwrap();
    let reads = AtomicUsize::new(0);
    assert!(!counted(&first.auth_freshness, &reads).await);
    assert!(!counted(&cloned.auth_freshness, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
    assert!(!counted(&separate.auth_freshness, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 2);
}

#[tokio::test(start_paused = true)]
async fn a_fresh_gate_does_not_bypass_credential_validation() {
    let provider = BedrockProvider::new(String::new()).unwrap();
    assert!(!counted(&provider.auth_freshness, &AtomicUsize::new(0)).await);
    let error = provider.prepare_auth().await.unwrap_err();
    assert_eq!(error.to_string(), "Bedrock API key is empty");
}
