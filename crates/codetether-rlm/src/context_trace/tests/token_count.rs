use super::*;

#[test]
fn estimate_tokens_approximation() {
    assert_eq!(ContextTrace::estimate_tokens("test"), 1);
    assert_eq!(ContextTrace::estimate_tokens("test test test test"), 4);
    assert_eq!(ContextTrace::estimate_tokens("12345678"), 2);
}
