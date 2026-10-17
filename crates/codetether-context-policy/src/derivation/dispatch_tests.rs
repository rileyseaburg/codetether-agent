//! Recovery precedence and policy payload regressions.
use super::*;
#[test]
fn preserves_selected_thresholds_and_budgets() {
    assert_eq!(
        select(
            DerivePolicy::Reset {
                threshold_tokens: 120
            },
            None
        ),
        Execution::Reset {
            threshold_tokens: 120
        }
    );
    assert_eq!(
        select(DerivePolicy::Incremental { budget_tokens: 90 }, None),
        Execution::Incremental { budget_tokens: 90 }
    );
    assert_eq!(
        select(DerivePolicy::OracleReplay { lookahead: 3 }, None),
        Execution::Legacy
    );
}
#[test]
fn forced_recovery_wins_even_when_zero() {
    for policy in [
        DerivePolicy::Legacy,
        DerivePolicy::Reset {
            threshold_tokens: 120,
        },
        DerivePolicy::Incremental { budget_tokens: 90 },
        DerivePolicy::OracleReplay { lookahead: 3 },
    ] {
        assert_eq!(select(policy, Some(0)), Execution::Legacy);
        assert_eq!(select(policy, Some(5)), Execution::Legacy);
    }
}
