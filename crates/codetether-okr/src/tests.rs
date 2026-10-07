//! KR progress and completion tests.

use super::*;

#[test]
fn test_key_result_progress() {
    let kr = KeyResult::new(Uuid::new_v4(), "Test KR", 100.0, "%");
    assert_eq!(kr.progress(), 0.0);

    let mut kr = kr;
    kr.update_progress(50.0);
    assert!((kr.progress() - 0.5).abs() < 0.001);
}

#[test]
fn test_key_result_progress_zero_target_achieved() {
    // "No critical errors" with target 0 and current 0 → achieved → 100 %
    let kr = KeyResult::new(Uuid::new_v4(), "No critical errors", 0.0, "count");
    assert!(kr.is_complete());
    assert_eq!(kr.progress(), 1.0);
}

#[test]
fn test_key_result_progress_zero_target_not_met() {
    // target 0 but errors occurred (current > 0) → 0 %
    let mut kr = KeyResult::new(Uuid::new_v4(), "No critical errors", 0.0, "count");
    kr.update_progress(3.0);
    assert!(!kr.is_complete());
    assert_eq!(kr.progress(), 0.0);
}
