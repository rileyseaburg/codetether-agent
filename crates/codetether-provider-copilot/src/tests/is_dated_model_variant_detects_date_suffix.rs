//! Existing regression test.
use super::*;

#[test]
fn is_dated_model_variant_detects_date_suffix() {
    assert!(is_dated_model_variant("gpt-4o-2024-05-13"));
    assert!(is_dated_model_variant("gpt-4o-2024-08-06"));
    assert!(is_dated_model_variant("gpt-4.1-2025-04-14"));
    assert!(is_dated_model_variant("gpt-4o-mini-2024-07-18"));
    assert!(!is_dated_model_variant("gpt-4o"));
    assert!(!is_dated_model_variant("gpt-5"));
    assert!(!is_dated_model_variant("claude-sonnet-4"));
    assert!(!is_dated_model_variant("gemini-2.5-pro"));
}
