//! Shared forage regression coverage.
use super::*;

#[test]
fn success_progress_increment_ratio_is_bounded() {
    let mut kr = KeyResult::new(Uuid::new_v4(), "KR", 100.0, "%");
    kr.update_progress(0.0);
    let high_remaining = success_progress_increment_ratio(&kr);
    assert!((high_remaining - 0.15).abs() < f64::EPSILON);

    kr.update_progress(95.0);
    let low_remaining = success_progress_increment_ratio(&kr);
    assert!((low_remaining - 0.05).abs() < f64::EPSILON);
}
