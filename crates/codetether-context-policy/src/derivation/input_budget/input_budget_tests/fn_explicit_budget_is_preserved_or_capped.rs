//! explicit budget is preserved or capped derivation policy.
use super::*;

#[test]
pub(super) fn explicit_budget_is_preserved_or_capped() {
    assert_eq!(resolve("gpt-4o", 32_000), 32_000);
    assert_eq!(resolve("mistral", 32_000), usable("mistral"));
}
