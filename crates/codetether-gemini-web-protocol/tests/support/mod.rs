//! Credential-free fixtures matching the observed array and framing layout.
use codetether_gemini_web_protocol::images::Continuation;
use serde_json::{Value, json};

pub fn state() -> Continuation {
    Continuation {
        conversation_id: "c_test".into(),
        response_id: "r_test".into(),
        choice_id: "rc_test".into(),
    }
}

pub fn candidate(urls: &[&str], id: &str) -> Value {
    let mut candidate = vec![Value::Null; 13];
    candidate[0] = json!(id);
    candidate[1] = json!(["Generated"]);
    let images: Vec<Value> = urls
        .iter()
        .map(|url| json!([[null, null, null, [null, null, null, url]]]))
        .collect();
    candidate[12] = json!([null, null, null, null, null, null, null, [images]]);
    json!(candidate)
}

pub fn inner(candidates: Vec<Value>) -> Value {
    json!([null, ["c_test", "r_test"], null, null, candidates])
}

pub fn frame(inner: Value) -> Value {
    json!(["wrb.fr", null, inner.to_string()])
}

pub fn batch(frames: &[Value]) -> Vec<u8> {
    let data = serde_json::to_string(frames).unwrap();
    format!("{}\n{data}\n", data.len() + 2).into_bytes()
}

pub fn body(frames: Vec<Value>) -> Vec<u8> {
    let mut result = b")]}'\n".to_vec();
    result.extend(batch(&frames));
    result
}
