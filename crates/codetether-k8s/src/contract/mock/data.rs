//! Request capture and configurable service behavior.
use serde_json::Value;
#[derive(Clone)]
pub struct RequestRecord {
    pub method: String,
    pub path: String,
    pub body: Value,
}
#[derive(Default)]
pub struct State {
    pub requests: Vec<RequestRecord>,
    pub lease: Value,
    pub pod: Value,
    pub conflict_once: bool,
    pub occupied: bool,
    pub missing_delete: bool,
}
pub fn error(code: u16) -> Value {
    serde_json::json!({"apiVersion":"v1","kind":"Status","status":"Failure",
        "message":"fixture error","reason":if code==409 {"AlreadyExists"} else {"NotFound"},"code":code})
}
