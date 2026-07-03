//! Tests for `Params` defaults, focusing on the `detach` flag (issue #296).

use super::params::Params;
use serde_json::json;

fn parse(value: serde_json::Value) -> Params {
    serde_json::from_value(value).expect("params parse")
}

#[test]
fn detach_defaults_to_true() {
    let p = parse(json!({ "action": "message", "name": "w", "message": "hi" }));
    assert!(
        p.detach_or_default(),
        "detach should default to true (#296)"
    );
}

#[test]
fn detach_explicit_false() {
    let p = parse(json!({
        "action": "message", "name": "w", "message": "hi", "detach": false
    }));
    assert!(!p.detach_or_default());
}

#[test]
fn detach_explicit_true() {
    let p = parse(json!({
        "action": "message", "name": "w", "message": "hi", "detach": true
    }));
    assert!(p.detach_or_default());
}
