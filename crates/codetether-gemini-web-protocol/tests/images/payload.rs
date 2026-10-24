use crate::support::state;
use codetether_gemini_web_protocol::images::build_payload;
use serde_json::{Value, json};
#[path = "payload/flags.rs"]
mod flags;
#[path = "payload/header.rs"]
mod header;
#[path = "payload/refine.rs"]
mod refine;

fn inner(prompt: &str, previous: bool) -> Value {
    let state = state();
    let encoded = build_payload(prompt, previous.then_some(&state)).unwrap();
    let outer: Value = serde_json::from_str(&encoded).unwrap();
    assert_eq!(outer[0], Value::Null);
    serde_json::from_str(outer[1].as_str().unwrap()).unwrap()
}

#[test]
fn image_payload_preserves_unicode_and_exact_slot_layout() {
    let prompt = "Draw 雪 ❄️ 🛶\n\"quoted\" & + %";
    let value = inner(prompt, false);
    assert_eq!(value.as_array().unwrap().len(), 99);
    assert_eq!(value[0], json!([prompt, 0, null, null, null, null, 0]));
    assert_eq!(value[1], json!(["en"]));
    assert_eq!(value[6], json!([0]));
    assert_eq!(value[17], json!([[1]]));
    assert_eq!(value[30], json!([4, 16, 19]));
    for slot in [54, 55, 61] {
        assert_eq!(value[slot], json!([]));
    }
    for slot in [2, 3, 4, 71] {
        assert!(value[slot].is_null());
    }
    assert!(uuid::Uuid::parse_str(value[59].as_str().unwrap()).is_ok());
    assert_ne!(value[59], inner("new", false)[59]);
}
