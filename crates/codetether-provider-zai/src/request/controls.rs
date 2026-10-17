//! Optional generation controls used by the injected-client request format.
use crate::CompletionRequest;
use serde_json::{Value, json};

pub(super) fn apply(body: &mut Value, request: &CompletionRequest) {
    body["stream"] = json!(false);
    if let Some(top_p) = request.top_p {
        body["top_p"] = json!(top_p);
    }
    if !request.stop.is_empty() {
        body["stop"] = json!(request.stop);
    }
}
