//! Regression tests for model-relative input budgets.

use super::{calculate, resolve, usable};

#[test]
fn calculation_scales_with_context_window() {
    assert_eq!(calculate(32_000, 8_192), 19_756);
    assert_eq!(calculate(128_000, 8_192), 105_984);
    assert_eq!(calculate(256_000, 8_192), 221_184);
    assert_eq!(calculate(1_000_000, 8_192), 890_784);
}

#[test]
fn zero_uses_selected_models_runtime_capacity() {
    assert_eq!(resolve("mistral", 0), usable("mistral"));
    assert_eq!(resolve("gpt-5.6-sol", 0), usable("gpt-5.6-sol"));
    assert!(usable("gpt-5.6-sol") > usable("mistral"));
}

#[test]
fn explicit_budget_is_preserved_or_capped() {
    assert_eq!(resolve("gpt-4o", 32_000), 32_000);
    assert_eq!(resolve("mistral", 32_000), usable("mistral"));
}
