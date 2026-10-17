//! project bucket escalates difficulty with error count policy.
use super::*;

#[test]
pub(super) fn project_bucket_escalates_difficulty_with_error_count() {
    let meta = RelevanceMeta {
        error_classes: vec!["error".into(), "failed".into(), "panicked".into()],
        ..Default::default()
    };
    assert_eq!(meta.project_bucket().difficulty, Difficulty::Hard);
}
