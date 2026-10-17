//! In-memory lease state transitions.
use super::data::{State, error};
use serde_json::{Value, json};
pub(super) fn respond(state: &mut State, method: &str, mut body: Value) -> (u16, Value) {
    if method == "POST" && state.occupied {
        return (409, error(409));
    }
    if method == "GET" {
        return (200, state.lease.clone());
    }
    if method == "DELETE" {
        return (
            200,
            json!({"kind":"Status","apiVersion":"v1","status":"Success"}),
        );
    }
    body["metadata"]["resourceVersion"] = json!("1");
    body["metadata"]["uid"] = json!("fixture-lease");
    state.lease = body.clone();
    (if method == "POST" { 201 } else { 200 }, body)
}
