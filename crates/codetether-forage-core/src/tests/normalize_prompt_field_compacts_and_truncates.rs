//! Shared forage regression coverage.
use super::*;

#[test]
fn normalize_prompt_field_compacts_and_truncates() {
    let input = "alpha   beta\n\n gamma    delta";
    assert_eq!(normalize_prompt_field(input, 128), "alpha beta gamma delta");

    let long = "x".repeat(400);
    let normalized = normalize_prompt_field(&long, 64);
    assert!(normalized.ends_with("...(truncated)"));
    assert!(normalized.len() > 64);
}
