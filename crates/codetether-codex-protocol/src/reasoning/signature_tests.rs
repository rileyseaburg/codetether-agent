use super::{decode, encode};
use serde_json::json;

#[test]
fn opaque_reasoning_item_round_trips() {
    let item = json!({
        "type": "reasoning",
        "id": "rs_1",
        "summary": [],
        "encrypted_content": "opaque"
    });
    let signature = encode(&item).expect("encoded reasoning");

    assert_eq!(decode(&signature), Some(item));
}

#[test]
fn rejects_plaintext_and_untrusted_shapes() {
    assert!(encode(&json!({"type": "reasoning"})).is_none());
    assert!(decode("not-a-codetether-signature").is_none());
}
