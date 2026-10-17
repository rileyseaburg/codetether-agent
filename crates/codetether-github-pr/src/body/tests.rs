use super::merge_signature;
use crate::context::SignatureContext;

#[test]
fn appends_signature_when_missing() {
    let signature = SignatureContext::sample();
    let body = merge_signature("hello", &signature);
    assert!(body.contains("## CodeTether Signature"));
    assert!(body.contains("Model: `zai/glm-5`"));
}

#[test]
fn replaces_existing_signature_block() {
    let signature = SignatureContext::sample();
    let body = merge_signature(
        "hello\n\n<!-- codetether-signature:start -->\nold\n<!-- codetether-signature:end -->",
        &signature,
    );
    assert!(body.contains("Agent ID: `user:test`"));
    assert!(!body.contains("\nold\n"));
}
