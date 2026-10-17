//! Match order and false-negative classification contract.
use crate::oracle::{GrepOracle, GrepVerification};
pub(super) fn check() {
    let oracle = GrepOracle::new("a\nb".into());
    let truth = vec![(1, "a".into()), (2, "b".into())];
    assert_eq!(
        oracle.verify_matches(&truth, &truth),
        GrepVerification::ExactMatch
    );
    assert_eq!(
        oracle.verify_matches(&[truth[1].clone(), truth[0].clone()], &truth),
        GrepVerification::UnorderedMatch
    );
    assert!(matches!(
        oracle.verify_matches(&truth[..1], &truth),
        GrepVerification::HasFalseNegatives { .. }
    ));
    assert!(matches!(
        oracle.verify_matches(&[(3, "c".into())], &truth),
        GrepVerification::Mismatch
    ));
}
