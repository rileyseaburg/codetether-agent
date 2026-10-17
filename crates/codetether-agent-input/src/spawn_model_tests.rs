//! Explicit and inherited spawn-model validation.
use crate::SpawnRequest;
use crate::test_helpers::params;
use serde_json::json;

#[test]
fn model_defaults_to_parent_current_model() {
    let p = params(json!({
        "action": "spawn",
        "name": "reviewer",
        "instructions": "audit the PR",
        "__ct_current_model": "zai/glm-5.1",
    }));
    let request = SpawnRequest::from_params(&p).expect("spawn request");
    assert_eq!(request.model, "zai/glm-5.1");
    assert_eq!(request.parent_session_id, None);
}

#[test]
fn explicit_model_takes_precedence() {
    let p = params(json!({
        "action": "spawn",
        "name": "reviewer",
        "instructions": "audit the PR",
        "model": "anthropic/claude-opus-4",
        "__ct_current_model": "zai/glm-5.1",
    }));
    let request = SpawnRequest::from_params(&p).expect("spawn request");
    assert_eq!(request.model, "anthropic/claude-opus-4");
}

#[test]
fn missing_both_models_errors() {
    let p = params(json!({
        "action": "spawn",
        "name": "reviewer",
        "instructions": "audit the PR",
    }));
    assert!(SpawnRequest::from_params(&p).is_err());
}
