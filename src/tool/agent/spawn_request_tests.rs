//! Tests for [`super::spawn_request::SpawnRequest::from_params`].
//!
//! Verifies that `model` defaults to the runtime-injected parent model
//! (`__ct_current_model`) so spawning works when the caller omits `model`.

use super::params::Params;
use super::spawn_request::SpawnRequest;
use serde_json::json;

fn params(value: serde_json::Value) -> Params {
    serde_json::from_value(value).expect("params parse")
}

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
