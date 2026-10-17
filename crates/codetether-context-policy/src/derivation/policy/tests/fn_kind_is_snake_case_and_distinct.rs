//! kind is snake case and distinct derivation policy.
use super::*;

#[test]
pub(super) fn kind_is_snake_case_and_distinct() {
    assert_eq!(DerivePolicy::Legacy.kind(), "legacy");
    assert_eq!(
        DerivePolicy::Reset {
            threshold_tokens: 0
        }
        .kind(),
        "reset"
    );
    assert_eq!(
        DerivePolicy::Incremental { budget_tokens: 0 }.kind(),
        "incremental"
    );
}
