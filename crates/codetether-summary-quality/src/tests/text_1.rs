use crate::clamp_tokens;

#[test]
fn clamps_long_text() {
    assert!(clamp_tokens(&"a".repeat(1000), 10).len() < 1000);
}
