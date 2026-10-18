//! Only explicit final decisions count as substantive verifier judgments.
use super::Verdict;

#[test]
fn malformed_or_missing_verdict_is_unavailable_not_a_rejection() {
    for report in [
        "",
        "looks fine",
        "VERDICT: PASS with gaps",
        "VERDICT: PASS\nextra text",
    ] {
        assert!(matches!(
            Verdict::parse(report),
            Verdict::Unavailable { .. }
        ));
    }
}

#[test]
fn exact_pass_and_fail_remain_distinct() {
    assert_eq!(Verdict::parse("VERDICT: PASS"), Verdict::Pass);
    assert!(matches!(
        Verdict::parse("VERDICT: FAIL"),
        Verdict::Fail { .. }
    ));
}
