//! Routing is harness metadata even when reports lie or concurrent runs finish out of order.

use super::{ObservationStore, RunState};
use std::sync::Arc;

#[test]
fn verifier_identity_survives_newer_run_and_model_self_claims() {
    let store = Arc::new(ObservationStore::default());
    let mut first = store.start();
    first.requested("provider/alias".into());
    first.resolved("provider", "actual-first");
    let mut second = store.start();
    second.requested("provider/next".into());
    second.resolved("provider", "actual-second");
    first.finish(&Ok("I am another model.\nVERDICT: PASS".into()));
    assert_eq!(first.identity(), "provider/actual-first");
    let latest = store.latest().unwrap();
    assert_eq!(latest.verification_id, second.id());
    assert!(matches!(latest.state, RunState::Running));
    second.finish(&Ok("VERDICT: FAIL".into()));
    assert!(matches!(store.latest().unwrap().state, RunState::Fail));
}

#[test]
fn verifier_identity_never_invents_resolution_for_cancelled_initialization() {
    let store = Arc::new(ObservationStore::default());
    {
        let mut attempt = store.start();
        attempt.requested("provider/requested".into());
    }
    let latest = store.latest().unwrap();
    assert_eq!(
        latest.requested_model.as_deref(),
        Some("provider/requested")
    );
    assert!(latest.resolved_provider.is_none());
    assert!(latest.resolved_model.is_none());
    assert!(latest.finished_at.is_some());
    assert!(matches!(latest.state, RunState::Unavailable));
}
