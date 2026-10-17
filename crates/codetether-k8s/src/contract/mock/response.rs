//! Deterministic API responses without network access.
use super::data::{State, error};
use serde_json::{Value, json};
pub fn respond(state: &mut State, method: &str, path: &str, body: Value) -> (u16, Value) {
    if path.contains("/leases") {
        return super::lease::respond(state, method, body);
    }
    if method == "POST" && state.conflict_once {
        state.conflict_once = false;
        return (409, error(409));
    }
    if method == "POST" {
        return (201, body);
    }
    if method == "DELETE" {
        return if state.missing_delete {
            (404, error(404))
        } else {
            (
                200,
                json!({"kind":"Status","apiVersion":"v1","status":"Success"}),
            )
        };
    }
    if path.contains("/deployments/") {
        return (
            200,
            json!({"kind":"Deployment","apiVersion":"apps/v1",
            "metadata":{"name":"controller"},"spec":{"replicas":3,"selector":{},"template":{"metadata":{},"spec":{"containers":[]}}},
            "status":{"availableReplicas":2}}),
        );
    }
    if state.pod.is_null() {
        (404, error(404))
    } else {
        (200, state.pod.clone())
    }
}
