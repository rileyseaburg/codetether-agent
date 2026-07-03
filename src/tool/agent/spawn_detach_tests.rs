//! Tests that the `detach` field on spawn params is correctly propagated to the
//! `SpawnRequest`, enabling background-first-turn dispatch (issue #295/#296).

use super::params::Params;
use super::spawn_request::SpawnRequest;
use serde_json::json;

fn params(value: serde_json::Value) -> Params {
    serde_json::from_value(value).expect("params parse")
}

#[test]
fn detach_defaults_to_true() {
    let p = params(json!({
        "action": "spawn",
        "name": "reviewer",
        "instructions": "audit the PR",
        "__ct_current_model": "zai/glm-5.1",
    }));
    let request = SpawnRequest::from_params(&p).expect("spawn request");
    assert!(request.detach, "detach should default to true (#296)");
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
