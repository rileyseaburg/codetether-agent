use super::*;
use crate::session::DerivePolicy;

fn overflow() -> anyhow::Error {
    anyhow::anyhow!("Your input exceeds the context window")
}

#[test]
fn incremental_prioritizes_prepared_context() {
    assert!(matches!(
        recovery(
            &overflow(),
            1,
            DerivePolicy::Incremental { budget_tokens: 0 }
        ),
        Some(Plan::Prepared {
            budget_tokens: 8_192
        })
    ));
    assert_eq!(
        recovery(
            &overflow(),
            2,
            DerivePolicy::Incremental { budget_tokens: 0 }
        ),
        Some(Plan::Emergency { keep_last: 6 })
    );
}

#[test]
fn explicit_legacy_starts_emergency_cascade() {
    assert_eq!(
        recovery(&overflow(), 1, DerivePolicy::Legacy),
        Some(Plan::Emergency { keep_last: 6 })
    );
}

#[test]
fn ignores_unrelated_errors() {
    let err = anyhow::anyhow!("connection reset");
    assert_eq!(recovery(&err, 1, DerivePolicy::default()), None);
}
