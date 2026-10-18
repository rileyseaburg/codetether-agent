use super::*;

fn r(policy: &'static str, tokens: usize, faults: usize) -> PolicyRunResult {
    PolicyRunResult {
        policy,
        kept_messages: 0,
        context_tokens: tokens,
        fault_count: faults,
        oracle_gap: 0,
        reuse_rate: 0.5,
    }
}

#[test]
fn flips_when_reset_dominates() {
    let res = vec![r("legacy", 24_000, 12), r("reset", 6_000, 4)];
    assert!(matches!(
        recommend_default(&res),
        Some(DerivePolicy::Reset { .. })
    ));
}

#[test]
fn keeps_legacy_when_no_dominator() {
    let res = vec![r("legacy", 24_000, 12), r("reset", 30_000, 4)];
    assert!(recommend_default(&res).is_none());
}
