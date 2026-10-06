//! Configuration previews follow the exact harness precedence, without execution claims.
use super::super::{defaults::Defaults, projection::snapshot, types::Source};

#[test]
fn verifier_model_api_precedence_and_unconfigured_state() {
    let defaults = || Defaults {
        environment: None,
        configured: Some("default/model".into()),
    };
    let state = snapshot(None, defaults(), Some("worker/other".into()), None);
    assert_eq!(state.source, Source::Default);
    assert_eq!(state.selected_model.as_deref(), Some("default/model"));
    let state = snapshot(None, defaults(), Some("worker/model".into()), None);
    assert_eq!(state.source, Source::Worker);
    assert_eq!(state.selected_model.as_deref(), Some("worker/model"));
    let state = snapshot(None, Defaults::default(), None, None);
    assert_eq!(state.source, Source::Unconfigured);
    assert!(state.selected_model.is_none());
    assert!(state.latest_verification.is_none());
}
