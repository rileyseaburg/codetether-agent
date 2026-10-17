//! Minimal Vault SDK response envelopes.
use serde_json::{Value, json};
pub fn lookup() -> Value {
    json!({"lease_id":"","lease_duration":0,"renewable":false,"request_id":"fixture","data":{
        "accessor":"fixture","creation_time":0,"creation_ttl":3600,"display_name":"fixture",
        "entity_id":"","expire_time":null,"explicit_max_ttl":0,"id":"fixture-token",
        "identity_policies":[],"issue_time":null,"meta":null,"num_uses":0,"orphan":false,
        "path":"auth/token/create","policies":["default"],"renewable":false,"ttl":3600}})
}
pub fn metadata() -> Value {
    json!({"created_time":"2026-10-17T00:00:00Z","deletion_time":"","destroyed":false,"version":1})
}
pub fn read(value: Value) -> Value {
    json!({"data":{"data":value,"metadata":metadata()}})
}
pub fn error() -> Value {
    json!({"errors":["fixture rejection"]})
}
pub fn login() -> Value {
    json!({"auth":{"client_token":"fixture-refreshed","accessor":"fixture","policies":["default"],
        "token_policies":["default"],"metadata":null,"lease_duration":3600,"renewable":false,
        "entity_id":"","token_type":"service","orphan":false}})
}

pub fn envelope(mut value: Value) -> Value {
    for (key, item) in [
        ("lease_id", json!("")),
        ("lease_duration", json!(0)),
        ("renewable", json!(false)),
        ("request_id", json!("fixture")),
    ] {
        value[key] = item;
    }
    value
}
