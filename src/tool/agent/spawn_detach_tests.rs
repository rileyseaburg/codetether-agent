//! Tests synchronous defaults and explicit background spawn dispatch.

use super::params::Params;
use super::spawn_request::SpawnRequest;
use serde_json::json;

fn params(value: serde_json::Value) -> Params {
    serde_json::from_value(value).expect("params parse")
}

#[test]
fn durable_spawn_defaults_to_synchronous() {
    let p = params(json!({
        "action": "spawn",
        "name": "reviewer",
        "instructions": "audit the PR",
        "__ct_current_model": "zai/glm-5.1",
    }));
    let request = SpawnRequest::from_params(&p).expect("spawn request");
    assert!(
        !request.detach,
        "model callers must receive the child result"
    );
}

#[test]
fn detach_false_is_propagated() {
    let p = params(json!({
        "action": "spawn",
        "name": "sync-agent",
        "instructions": "work on issue #213",
        "model": "zai/glm-5.1",
        "detach": false,
    }));
    let request = SpawnRequest::from_params(&p).expect("spawn request");
    assert!(
        !request.detach,
        "detach should be false when explicitly set"
    );
}

#[test]
fn ephemeral_spawn_defaults_to_synchronous() {
    let p = params(json!({
        "action": "spawn", "name": "once", "instructions": "inspect",
        "model": "zai/glm-5.1", "ephemeral": true,
    }));
    assert!(!SpawnRequest::from_params(&p).unwrap().detach);
}
