//! Tests for [`crate::provider::vertex_anthropic_catalog::catalog`].

use super::catalog;

#[test]
fn exposes_opus_5_first() {
    let models = catalog();
    assert_eq!(models[0].id, "claude-opus-5");
    assert_eq!(models[0].provider, "vertex-anthropic");
}

#[test]
fn opus_5_has_million_token_context() {
    let models = catalog();
    let opus5 = models.iter().find(|m| m.id == "claude-opus-5").unwrap();
    assert_eq!(opus5.context_window, 1_000_000);
    assert_eq!(opus5.max_output_tokens, Some(128_000));
    assert!(opus5.supports_tools && opus5.supports_vision);
}

#[test]
fn older_opus_keeps_200k_window() {
    let models = catalog();
    let opus4 = models
        .iter()
        .find(|m| m.id == "claude-opus-4-20250514")
        .unwrap();
    assert_eq!(opus4.context_window, 200_000);
}
